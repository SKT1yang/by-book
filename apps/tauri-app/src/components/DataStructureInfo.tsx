import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

const DataStructureInfo: React.FC = () => {
  const [structure, setStructure] = useState<string[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadStructure();
  }, []);

  async function loadStructure() {
    try {
      setLoading(true);
      setError(null);
      const result = await invoke<string[]>("get_app_data_structure");
      setStructure(result);
    } catch (err) {
      setError("无法加载应用数据结构信息: " + err);
    } finally {
      setLoading(false);
    }
  }

  if (loading) {
    return <div>正在加载应用数据结构信息...</div>;
  }

  if (error) {
    return <div className="error-message">{error}</div>;
  }

  return (
    <div className="data-structure-info">
      <h3>应用数据目录结构</h3>
      <div className="structure-list">
        {structure.map((item, index) => (
          <div key={index} className="structure-item">
            {item}
          </div>
        ))}
      </div>
      <button onClick={loadStructure}>刷新</button>
    </div>
  );
};

export default DataStructureInfo;