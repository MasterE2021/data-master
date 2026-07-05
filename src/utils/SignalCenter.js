// src/utils/SignalCenter.js
import Logger from './Logger.js'; // 引入刚才写的日志系统

/*

------------------ 1. 发送方 ------------------
import Signal from './utils/SignalCenter.js'

function clickBtn() {
  // 最后加上一个带 __source 的对象，系统就会识别来源
  Signal.emit('btn-file', '点击了文件按钮', new Date(), { __source: 'Home.vue' });
}

// ------------------ 2. 接收方 ------------------
import { onMounted, onUnmounted } from 'vue';
import Signal from './utils/SignalCenter.js';

// 接收到信号后的处理逻辑
const handleFileBtnClick = (msg, time) => {
  console.log('FileSidebar 收到了通知:', msg, '时间:', time);
};

onMounted(() => {
  // 订阅信号，第三个参数填组件名，方便日志追踪
  Signal.on('btn-file', handleFileBtnClick, 'FileSidebar.vue');
});

onUnmounted(() => {
  // 组件销毁时必须取消订阅
  Signal.off('btn-file', handleFileBtnClick);
});

*/

/**
 * 信号中心系统 (Event Bus)
 * 具备完善的日志追踪能力
 */
class SignalCenter {
    constructor() {
        this.events = new Map();
        // 忽略日志记录的内部信号（可以保留作为第一层防御）
        this.ignoreLogEvents = new Set(['SYSTEM_LOG']);

        // ✨ 新增：防重入锁，彻底避免死循环
        this._isTracing = false;
    }

    /**
     * 内部方法：安全地打印信号日志
     */
    _traceLog(action, eventName, details) {
        // 1. 常规判断跳过
        if (this.ignoreLogEvents.has(eventName)) return;

        // 2. 防重入判断：如果当前正在打印日志，直接跳过，防止互相嵌套调用
        if (this._isTracing) return;

        this._isTracing = true; // 上锁
        try {
            // 格式化输出，方便在终端和控制台阅读
            Logger.debug(`[Signal | ${action}] <${eventName}>`, details);
        } finally {
            this._isTracing = false; // 解锁
        }
    }

    /**
     * 1. 订阅信号
     * @param {string} eventName - 信号名称
     * @param {Function} callback - 回调函数
     * @param {Object|string} [context=null] - 订阅者身份/上下文标识 (建议传组件名,如 'Header.vue')
     */
    on(eventName, callback, context = null) {
        if (typeof callback !== 'function') {
            Logger.warn(`[Signal] 订阅 <${eventName}> 失败: callback 不是函数`);
            return;
        }

        if (!this.events.has(eventName)) {
            this.events.set(eventName, []);
        }

        this.events.get(eventName).push({callback, context});

        // 记录订阅日志
        const subscriber = typeof context === 'string' ? context : (context?.name || 'Unknown');
        this._traceLog('Subscribe', eventName, `订阅者: ${subscriber}`);
    }

    /**
     * 2. 发送信号
     * @param {string} eventName - 信号名称
     * @param {...any} args - 传递给回调函数的参数
     */
    emit(eventName, ...args) {
        // 提取发出信号的源头（约定 args 的最后一位如果是带 __source 的对象，则作为发送源）
        let source = 'Unknown';
        let payload = args;

        // 可选：允许在发送时标识发送者，如: Signal.emit('LOGIN', data, { __source: 'LoginButton' })
        const lastArg = args[args.length - 1];
        if (lastArg && typeof lastArg === 'object' && lastArg.__source) {
            source = lastArg.__source;
            payload = args.slice(0, -1); // 从日志的真实数据中剔除掉标记位
        }

        if (!this.events.has(eventName)) {
            this._traceLog('Emit (No Sub)', eventName, {from: source, data: payload, warning: '无订阅者'});
            return false;
        }

        const handlers = [...this.events.get(eventName)];

        // 记录发送日志
        this._traceLog('Emit', eventName, {from: source, handlerCount: handlers.length, data: payload});

        handlers.forEach(handler => {
            try {
                // 记录接收与执行日志
                const receiver = typeof handler.context === 'string' ? handler.context : (handler.context?.name || 'Unknown');
                this._traceLog('Receive', eventName, `目标执行: ${receiver}`);

                handler.callback.apply(handler.context, args);
            } catch (error) {
                Logger.error(`[Signal | Error] 执行 <${eventName}> 回调时崩溃:`, error);
            }
        });

        return true;
    }

    /**
     * 3. 取消订阅
     */
    off(eventName, callback) {
        if (!this.events.has(eventName)) return;

        if (!callback) {
            this.events.delete(eventName);
            this._traceLog('Unsubscribe', eventName, '已清空该信号下的所有订阅');
            return;
        }

        const handlers = this.events.get(eventName);
        const remainingHandlers = handlers.filter(handler => handler.callback !== callback && handler.callback.originalCallback !== callback);

        if (remainingHandlers.length === 0) {
            this.events.delete(eventName);
            this._traceLog('Unsubscribe', eventName, '最后一个回调已移除，信号注销');
        } else {
            this.events.set(eventName, remainingHandlers);
            this._traceLog('Unsubscribe', eventName, `移除单个回调，剩余 ${remainingHandlers.length} 个订阅者`);
        }
    }

    /**
     * 4. 订阅一次
     */
    once(eventName, callback, context = null) {
        const wrapper = (...args) => {
            this.off(eventName, wrapper);
            callback.apply(context, args);
        };
        wrapper.originalCallback = callback;
        this.on(eventName, wrapper, context);
    }

    /**
     * 5. 清空所有信号
     */
    clearAll() {
        this.events.clear();
        Logger.warn('[Signal] 清空了所有系统信号');
    }
}

const globalSignalCenter = new SignalCenter();
export default globalSignalCenter;