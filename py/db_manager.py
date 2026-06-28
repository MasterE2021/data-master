# db_manager.py

import sys
import os
import sqlite3
from typing import Any

import duckdb


class SQLiteManager:
    """
    基于 SQLite 的文件路径持久化管理器。
    负责创建/读取用户目录下的 .data-x/dx-file.data 文件，
    提供记录增删查等操作。
    """

    def __init__(self, db_path):
        # 允许多线程共享连接（FastAPI 需要）
        self.conn = sqlite3.connect(db_path, check_same_thread=False)
        # 启用 WAL 模式提升并发写入性能
        self.conn.execute("PRAGMA journal_mode=WAL")
        # 初始化数据表
        self._init_tables()

    def _init_tables(self):
        # 创建导入历史记录表
        sql = """
        CREATE TABLE IF NOT EXISTS import_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT NOT NULL,
            name TEXT NOT NULL,
            import_time DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        """
        self.exec_sql(sql)

    def exec_sql(self, sql: str, params: tuple = ()):
        self.conn.execute(sql, params)
        self.conn.commit()

    def run_sql(self, sql: str, params: tuple = ()) -> list[Any]:
        cursor = self.conn.execute(sql, params)
        return cursor.fetchall()


class DuckDBManager:
    """DuckDB 内存模式管理器，用于高效查询数据文件（Excel/CSV/Parquet 等）。
    自动加载离线插件以确保在只读环境（如 PyInstaller exe）中可用。
    """

    def __init__(self):
        self.conn = duckdb.connect()
        self._load_offline_extensions()

    def _load_offline_extensions(self):
        required_extensions = ["excel.duckdb_extension", "postgres_scanner.duckdb_extension"]

        # 自适应获取插件根目录：exe 时用 sys._MEIPASS，否则向上推导至项目根
        root_dir = sys._MEIPASS if getattr(sys, 'frozen', False) else os.path.dirname(
            os.path.dirname(os.path.abspath(__file__)))
        ext_dir = os.path.join(root_dir, "lib/duckdb_extension")

        for ext_name in required_extensions:
            ext_path = os.path.join(ext_dir, ext_name)
            if not os.path.exists(ext_path):
                raise FileNotFoundError(f"缺少关键离线插件文件: '{ext_name}'，路径应为: '{ext_path}'")

            safe_path = ext_path.replace('\\', '/')
            self.conn.query(f"load '{safe_path}'")

        loaded_ext = self.conn.query(
            "select extension_name from duckdb_extensions() where loaded = true"
        ).fetchall()
        print(f"--- DuckDB 已成功加载的插件: {[row[0] for row in loaded_ext]} ---")

    def query(self, sql_query):
        """统一查询接口，返回 DuckDB 结果对象。"""
        return self.conn.query(sql_query)