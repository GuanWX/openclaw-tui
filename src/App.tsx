import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

interface TokenBalance {
  provider: string;
  balance: number | null;
  used: number | null;
  limit: number | null;
  error: string | null;
  lastUpdated: string | null;
}

function App() {
  const [balances, setBalances] = useState<TokenBalance[]>([
    { provider: "OpenAI", balance: null, used: null, limit: null, error: null, lastUpdated: null },
    { provider: "Copilot", balance: null, used: null, limit: null, error: null, lastUpdated: null },
  ]);
  const [loading, setLoading] = useState(false);

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

  useEffect(() => {
    fetchBalances();
    // Auto-refresh every 5 minutes
    const interval = setInterval(fetchBalances, 5 * 60 * 1000);
    return () => clearInterval(interval);
  }, []);

  const formatNumber = (num: number | null) => {
    if (num === null) return "—";
    return num.toLocaleString();
  };

  const formatCurrency = (num: number | null) => {
    if (num === null) return "—";
    return `$${num.toFixed(2)}`;
  };

  return (
    <div className="container">
      <h1>🤖 AI Token Monitor</h1>

      {balances.map((item, index) => (
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
        <a href="#" onClick={(e) => { e.preventDefault(); invoke("open_settings"); }}>
          ⚙️ Settings
        </a>
      </div>
    </div>
  );
}

export default App;