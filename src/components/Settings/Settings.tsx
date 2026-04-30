import React, { useState } from 'react';
import {
  Sun,
  Moon,
  Key,
  Server,
  Keyboard,
  Download,
  Trash2,
  Info,
  Save,
  Eye,
  EyeOff,
} from 'lucide-react';
import { useSettingsStore } from '@/stores/settingsStore';
import { Button } from '@/components/Common/Button';
import styles from './Settings.module.css';

export const Settings: React.FC = () => {
  const settings = useSettingsStore();
  const [activeTab, setActiveTab] = useState('ai');
  const [showApiKey, setShowApiKey] = useState(false);
  const [saved, setSaved] = useState(false);

  const handleSave = () => {
    setSaved(true);
    setTimeout(() => setSaved(false), 2000);
  };

  const tabs = [
    { id: 'ai', label: 'AI 配置', icon: Key },
    { id: 'sync', label: '同步', icon: Server },
    { id: 'hotkeys', label: '热键', icon: Keyboard },
    { id: 'appearance', label: '外观', icon: Sun },
    { id: 'data', label: '数据', icon: Download },
    { id: 'about', label: '关于', icon: Info },
  ];

  return (
    <div className={styles.container}>
      <h1 className={styles.pageTitle}>设置</h1>

      <div className={styles.content}>
        {/* Tabs */}
        <nav className={styles.tabs}>
          {tabs.map((tab) => (
            <button
              key={tab.id}
              className={`${styles.tab} ${activeTab === tab.id ? styles.tabActive : ''}`}
              onClick={() => setActiveTab(tab.id)}
            >
              <tab.icon size={16} />
              {tab.label}
            </button>
          ))}
        </nav>

        {/* Tab Content */}
        <div className={styles.tabContent}>
          {/* AI Config */}
          {activeTab === 'ai' && (
            <div className={styles.section}>
              <h2 className={styles.sectionTitle}>AI 配置</h2>
              <p className={styles.sectionDesc}>
                配置 AI 模型用于截屏内容分析和任务提取
              </p>

              <div className={styles.formGroup}>
                <label className={styles.label}>API Key</label>
                <div className={styles.inputWithBtn}>
                  <input
                    className={styles.input}
                    type={showApiKey ? 'text' : 'password'}
                    value={settings.aiConfig.api_key}
                    onChange={(e) => settings.updateAiConfig({ api_key: e.target.value })}
                    placeholder="输入你的 API Key"
                  />
                  <button
                    className={styles.inputBtn}
                    onClick={() => setShowApiKey(!showApiKey)}
                    type="button"
                  >
                    {showApiKey ? <EyeOff size={16} /> : <Eye size={16} />}
                  </button>
                </div>
              </div>

              <div className={styles.formGroup}>
                <label className={styles.label}>模型</label>
                <select
                  className={styles.select}
                  value={settings.aiConfig.model}
                  onChange={(e) => settings.updateAiConfig({ model: e.target.value })}
                >
                  <option value="gpt-4o">GPT-4o</option>
                  <option value="gpt-4o-mini">GPT-4o Mini</option>
                  <option value="gpt-4-turbo">GPT-4 Turbo</option>
                  <option value="gpt-3.5-turbo">GPT-3.5 Turbo</option>
                  <option value="claude-3-opus">Claude 3 Opus</option>
                  <option value="claude-3-sonnet">Claude 3 Sonnet</option>
                </select>
              </div>

              <div className={styles.formGroup}>
                <label className={styles.label}>API Endpoint</label>
                <input
                  className={styles.input}
                  value={settings.aiConfig.api_endpoint}
                  onChange={(e) => settings.updateAiConfig({ api_endpoint: e.target.value })}
                  placeholder="https://api.openai.com/v1"
                />
              </div>

              <div className={styles.formRow}>
                <div className={styles.formGroup}>
                  <label className={styles.label}>Max Tokens</label>
                  <input
                    className={styles.input}
                    type="number"
                    value={settings.aiConfig.max_tokens}
                    onChange={(e) =>
                      settings.updateAiConfig({ max_tokens: parseInt(e.target.value) || 2000 })
                    }
                  />
                </div>
                <div className={styles.formGroup}>
                  <label className={styles.label}>Temperature</label>
                  <input
                    className={styles.input}
                    type="number"
                    step="0.1"
                    min="0"
                    max="2"
                    value={settings.aiConfig.temperature}
                    onChange={(e) =>
                      settings.updateAiConfig({ temperature: parseFloat(e.target.value) || 0.3 })
                    }
                  />
                </div>
              </div>

              <Button variant="primary" onClick={handleSave} icon={<Save size={14} />}>
                {saved ? '已保存' : '保存配置'}
              </Button>
            </div>
          )}

          {/* Sync Config */}
          {activeTab === 'sync' && (
            <div className={styles.section}>
              <h2 className={styles.sectionTitle}>同步配置</h2>
              <p className={styles.sectionDesc}>
                配置多设备间的任务数据同步
              </p>

              <div className={styles.formGroup}>
                <label className={styles.label}>服务器地址</label>
                <input
                  className={styles.input}
                  value={settings.syncConfig.server_url}
                  onChange={(e) => settings.updateSyncConfig({ server_url: e.target.value })}
                  placeholder="https://your-sync-server.com"
                />
              </div>

              <div className={styles.formRow}>
                <div className={styles.formGroup}>
                  <label className={styles.label}>同步间隔（分钟）</label>
                  <input
                    className={styles.input}
                    type="number"
                    value={settings.syncConfig.sync_interval}
                    onChange={(e) =>
                      settings.updateSyncConfig({ sync_interval: parseInt(e.target.value) || 30 })
                    }
                  />
                </div>
                <div className={styles.formGroup}>
                  <label className={styles.label}>设备 ID</label>
                  <input
                    className={styles.input}
                    value={settings.syncConfig.device_id}
                    onChange={(e) => settings.updateSyncConfig({ device_id: e.target.value })}
                    placeholder="自动生成"
                  />
                </div>
              </div>

              <div className={styles.toggleGroup}>
                <label className={styles.toggleLabel}>自动同步</label>
                <button
                  className={`${styles.toggle} ${settings.syncConfig.auto_sync ? styles.toggleOn : ''}`}
                  onClick={() =>
                    settings.updateSyncConfig({ auto_sync: !settings.syncConfig.auto_sync })
                  }
                >
                  <span className={styles.toggleThumb} />
                </button>
              </div>

              <Button variant="primary" onClick={handleSave} icon={<Save size={14} />}>
                {saved ? '已保存' : '保存配置'}
              </Button>
            </div>
          )}

          {/* Hotkeys */}
          {activeTab === 'hotkeys' && (
            <div className={styles.section}>
              <h2 className={styles.sectionTitle}>热键配置</h2>
              <p className={styles.sectionDesc}>自定义快捷键</p>

              {Object.entries(settings.hotkeys).map(([action, key]) => (
                <div key={action} className={styles.hotkeyItem}>
                  <span className={styles.hotkeyLabel}>
                    {action === 'screenshot' && '截屏'}
                    {action === 'newTask' && '新建任务'}
                    {action === 'search' && '搜索'}
                    {action === 'toggleSidebar' && '切换侧边栏'}
                  </span>
                  <kbd className={styles.hotkeyValue}>{key}</kbd>
                </div>
              ))}
            </div>
          )}

          {/* Appearance */}
          {activeTab === 'appearance' && (
            <div className={styles.section}>
              <h2 className={styles.sectionTitle}>外观</h2>
              <p className={styles.sectionDesc}>自定义应用外观</p>

              <div className={styles.themeOptions}>
                <button
                  className={`${styles.themeOption} ${settings.theme === 'light' ? styles.themeActive : ''}`}
                  onClick={() => settings.setTheme('light')}
                >
                  <Sun size={20} />
                  <span>浅色</span>
                </button>
                <button
                  className={`${styles.themeOption} ${settings.theme === 'dark' ? styles.themeActive : ''}`}
                  onClick={() => settings.setTheme('dark')}
                >
                  <Moon size={20} />
                  <span>深色</span>
                </button>
              </div>

              <div className={styles.formGroup}>
                <label className={styles.label}>悬浮卡片透明度</label>
                <input
                  className={styles.range}
                  type="range"
                  min="0.3"
                  max="1"
                  step="0.1"
                  value={settings.floatingCardOpacity}
                  onChange={(e) =>
                    settings.updateSettings({ floatingCardOpacity: parseFloat(e.target.value) })
                  }
                />
                <span className={styles.rangeValue}>
                  {Math.round(settings.floatingCardOpacity * 100)}%
                </span>
              </div>
            </div>
          )}

          {/* Data */}
          {activeTab === 'data' && (
            <div className={styles.section}>
              <h2 className={styles.sectionTitle}>数据管理</h2>
              <p className={styles.sectionDesc}>导出或清除你的数据</p>

              <div className={styles.dataActions}>
                <Button variant="secondary" icon={<Download size={14} />}>
                  导出数据
                </Button>
                <Button variant="danger" icon={<Trash2 size={14} />}>
                  清除所有数据
                </Button>
              </div>
            </div>
          )}

          {/* About */}
          {activeTab === 'about' && (
            <div className={styles.section}>
              <h2 className={styles.sectionTitle}>关于 SnapTask</h2>
              <div className={styles.aboutInfo}>
                <div className={styles.aboutRow}>
                  <span className={styles.aboutLabel}>版本</span>
                  <span className={styles.aboutValue}>0.1.0</span>
                </div>
                <div className={styles.aboutRow}>
                  <span className={styles.aboutLabel}>描述</span>
                  <span className={styles.aboutValue}>截屏即落库的 AI 智能任务管理系统</span>
                </div>
                <div className={styles.aboutRow}>
                  <span className={styles.aboutLabel}>技术栈</span>
                  <span className={styles.aboutValue}>
                    React + TypeScript + Tauri + Rust
                  </span>
                </div>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
