import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

interface TokenBalance {
  provider: string;
  balance: number | null;
  used: number | null;
  limit: number | null;
  error: string | null;
  lastUpdated: string | null;
}

interface Config {
  openai_api_key: string | null;
  github_token: string | null;
  refresh_interval: number;
}

function App() {
  const [balances, setBalances] = useState<TokenBalance[]>([
    { provider: "OpenAI", balance: null, used: null, limit: null, error: null, lastUpdated: null },
    { provider: "Copilot", balance: null, used: null, limit: null, error: null, lastUpdated: null },
  ]);
  const [loading, setLoading] = useState(false);
  const [showSettings, setShowSettings] = useState(false);
  const [config, setConfig] = useState<Config>({
    openai_api_key: "",
    github_token: "",
    refresh_interval: 300,
  });
  const [saving, setSaving] = useState(false);
  const [saveMessage, setSaveMessage] = useState("");

  const fetchBalances = async () => {
    setLoading(true);
    try {
      const result = await invoke<TokenBalance[]>("fetch_all_balances");
      setBalances(result);
    } catch (error) {
      console.error("Failed to fetch balances:", error);
    } finally {
      setLoading(false);
    }
  };

  const loadConfig = async () => {
    try {
      const cfg = await invoke<Config>("get_config");
      setConfig({
        openai_api_key: cfg.openai_api_key || "",
        github_token: cfg.github_token || "",
        refresh_interval: cfg.refresh_interval,
      });
    } catch (error) {
      console.error("Failed to load config:", error);
    }
  };

  const saveConfig = async () => {
    setSaving(true);
    setSaveMessage("");
    try {
      await invoke("save_config", {
        openaiApiKey: config.openai_api_key || null,
        githubToken: config.github_token || null,
        refreshInterval: config.refresh_interval,
      });
      setSaveMessage("✅ 保存成功！");
      setTimeout(() => {
        setShowSettings(false);
        fetchBalances();
      }, 1000);
    } catch (error) {
      setSaveMessage(`❌ 保存失败: ${error}`);
    } finally {
      setSaving(false);
    }
  };

  useEffect(() => {
    fetchBalances();
    loadConfig();

    // 监听打开设置事件
    const unlisten = listen("open-settings", () => {
      setShowSettings(true);
      loadConfig();
    });

    // Auto-refresh
    const interval = setInterval(fetchBalances, 5 * 60 * 1000);

    return () => {
      unlisten.then((fn) => fn());
      clearInterval(interval);
    };
  }, []);

  const formatNumber = (num: number | null) => {
    if (num === null) return "—";
    return num.toLocaleString();
  };

  const formatCurrency = (num: number | null) => {
    if (num === null) return "—";
    return `$${num.toFixed(2)}`;
  };

  // 配置页面
  if (showSettings) {
    return (
      <div className="container">
        <h1>⚙️ 配置</h1>

        <div className="settings-form">
          <div className="form-group">
            <label>OpenAI API Key</label>
            <input
              type="password"
              placeholder="sk-..."
              value={config.openai_api_key || ""}
              onChange={(e) =>
                setConfig({ ...config, openai_api_key: e.target.value })
              }
            />
            <small>用于查询 OpenAI API 余额</small>
          </div>

          <div className="form-group">
            <label>GitHub Token (Copilot)</label>
            <input
              type="password"
              placeholder="ghp_..."
              value={config.github_token || ""}
              onChange={(e) =>
                setConfig({ ...config, github_token: e.target.value })
              }
            />
            <small>用于查询 GitHub Copilot 用量</small>
          </div>

          <div className="form-group">
            <label>刷新间隔 (秒)</label>
            <input
              type="number"
              min={60}
              value={config.refresh_interval}
              onChange={(e) =>
                setConfig({
                  ...config,
                  refresh_interval: parseInt(e.target.value) || 300,
                })
              }
            />
          </div>

          {saveMessage && <div className="save-message">{saveMessage}</div>}

          <div className="button-row">
            <button
              className="cancel-btn"
              onClick={() => setShowSettings(false)}
            >
              取消
            </button>
            <button
              className="save-btn"
              onClick={saveConfig}
              disabled={saving}
            >
              {saving ? "保存中..." : "💾 保存"}
            </button>
          </div>
        </div>
      </div>
    );
  }

  // 主页面
  return (
    <div className="container">
      <h1>🤖 AI Token Monitor</h1>

      {balances.map((item) => (
        <div key={item.provider} className="card">
          <div className="card-header">
            <span className="provider-name">
              {item.provider === "OpenAI" ? "🟢" : "🟣"} {item.provider}
            </span>
            <span
              className={`status-dot ${
                loading ? "loading" : item.error ? "error" : "success"
              }`}
            />
          </div>

          <div className="balance-info">
            {item.provider === "OpenAI" ? (
              <>
                <div className="balance-row">
                  <span className="balance-label">Credits Remaining</span>
                  <span className="balance-value">
                    {formatCurrency(item.balance)}
                  </span>
                </div>
                <div className="balance-row">
                  <span className="balance-label">Used This Month</span>
                  <span className="balance-value">
                    {formatCurrency(item.used)}
                  </span>
                </div>
              </>
            ) : (
              <>
                <div className="balance-row">
                  <span className="balance-label">Tokens Used</span>
                  <span className="balance-value">{formatNumber(item.used)}</span>
                </div>
                <div className="balance-row">
                  <span className="balance-label">Monthly Limit</span>
                  <span className="balance-value">{formatNumber(item.limit)}</span>
                </div>
              </>
            )}

            {item.lastUpdated && (
              <div className="balance-row" style={{ fontSize: "0.75rem", color: "#666" }}>
                <span>Updated: {item.lastUpdated}</span>
              </div>
            )}

            {item.error && <div className="error-message">{item.error}</div>}
          </div>
        </div>
      ))}

      <button className="refresh-btn" onClick={fetchBalances} disabled={loading}>
        {loading ? "Refreshing..." : "🔄 Refresh"}
      </button>

      <div className="settings-link">
        <button
          className="settings-btn"
          onClick={() => {
            setShowSettings(true);
            loadConfig();
          }}
        >
          ⚙️ 配置
        </button>
      </div>
    </div>
  );
}

export default App;