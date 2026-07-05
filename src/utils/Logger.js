// src/utils/Logger.js
import Signal from './SignalCenter.js';
import {SignalName} from './Common.js';
import {invoke} from '@tauri-apps/api/core'; // Tauri 2.0 API

/* 使用示例

import Logger from './utils/Logger.js';

const testLogs = () => {
  Logger.debug('正在加载工作空间数据...');
  Logger.info('Python 后端连接成功', { port: 8000, status: 'ok' });
  Logger.warn('DuckDB 插件加载延迟');
  Logger.error('数据库查询失败', 'Table "users" not found');
};


*/

// 判断当前是否为生产环境 (发布版本)
// 适配 Vite 环境。如果你使用的是 Webpack (Vue CLI)，请替换为: process.env.NODE_ENV === 'production'
const isProd = typeof import.meta !== 'undefined' && import.meta.env && import.meta.env.PROD;
console.log('当前环境为生产环境吗:' + isProd)

// 定义日志级别和对应的颜色
const LOG_LEVELS = {
    DEBUG: {value: 1, color: '#808080', label: 'DEBUG'},
    INFO: {value: 2, color: '#00BFFF', label: 'INFO '},
    WARN: {value: 3, color: '#FFA500', label: 'WARN '},
    ERROR: {value: 4, color: '#FF0000', label: 'ERROR'}
};

class LoggerSystem {
    // 构造函数动态赋予默认日志级别
    constructor(currentLevel = null) {
        if (currentLevel !== null) {
            this.currentLevel = currentLevel;
        } else {
            // 生产发布版默认只打印 WARN 和 ERROR (3)
            // 开发环境默认打印所有日志 (1)
            this.currentLevel = isProd ? LOG_LEVELS.WARN.value : LOG_LEVELS.DEBUG.value;
        }
    }

    // 格式化时间
    _getTimestamp() {
        const now = new Date();
        return `${now.getHours().toString().padStart(2, '0')}:${now.getMinutes().toString().padStart(2, '0')}:${now.getSeconds().toString().padStart(2, '0')}.${now.getMilliseconds().toString().padStart(3, '0')}`;
    }

    // 将复杂参数转为字符串，供 Rust 后端打印
    _stringifyArgs(args) {
        return args.map(arg => {
            if (typeof arg === 'object') {
                try {
                    return JSON.stringify(arg);
                } catch (e) {
                    return '[Unserializable Object]';
                }
            }
            return String(arg);
        }).join(' ');
    }

    /**
     * 核心日志处理方法
     */
    _log(levelObj, ...args) {
        // 【关键防御】如果当前日志级别低于系统设定的级别，直接跳过
        // 这样在生产环境下，DEBUG 和 INFO 不会执行后续的序列化、打印和 Tauri 跨进程通信，实现零消耗
        if (levelObj.value < this.currentLevel) return;

        const timestamp = this._getTimestamp();
        const strMessage = this._stringifyArgs(args);

        // 1. 浏览器控制台打印
        const browserStyle = `color: ${levelObj.color}; font-weight: bold;`;
        console.log(`%c[${timestamp}] [${levelObj.label}]`, browserStyle, ...args);

        // 2. Tauri 双端打印：发送给 Rust 后端终端
        if (window.__TAURI_INTERNALS__) {
            invoke('backend_log', {
                level: levelObj.label.trim(), timestamp: timestamp, message: strMessage
            }).catch(err => {
                console.error('Failed to send log to Tauri backend:', err);
            });
        }
    }

    debug(...args) {
        this._log(LOG_LEVELS.DEBUG, ...args);
    }

    info(...args) {
        this._log(LOG_LEVELS.INFO, ...args);
    }

    warn(...args) {
        this._log(LOG_LEVELS.WARN, ...args);
    }

    error(...args) {
        this._log(LOG_LEVELS.ERROR, ...args);
    }
}

const Logger = new LoggerSystem();
export default Logger;