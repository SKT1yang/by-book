import React, { useState, useEffect, useRef } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { invoke } from "@tauri-apps/api/core";

const Reader: React.FC = () => {
  const { filename } = useParams<{ filename: string }>();
  const navigate = useNavigate();
  const [content, setContent] = useState("");
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [fontSize, setFontSize] = useState(16);
  const [theme, setTheme] = useState("light");
  const contentRef = useRef<HTMLDivElement>(null);

  // 加载文档内容
  useEffect(() => {
    if (filename) {
      loadDocument(filename);
    }
  }, [filename]);

  async function loadDocument(filename: string) {
    try {
      setLoading(true);
      const content = await invoke<string>("load_document", { filename });
      setContent(content);
      setError("");
    } catch (err) {
      setError("加载文档失败: " + err);
    } finally {
      setLoading(false);
    }
  }

  // 返回书架
  const goBack = () => {
    navigate("/");
  };

  // 增大字体
  const increaseFontSize = () => {
    setFontSize(prev => Math.min(prev + 1, 24));
  };

  // 减小字体
  const decreaseFontSize = () => {
    setFontSize(prev => Math.max(prev - 1, 12));
  };

  // 切换主题
  const toggleTheme = () => {
    setTheme(prev => prev === "light" ? "dark" : "light");
  };

  // 上一页
  const prevPage = () => {
    if (contentRef.current) {
      contentRef.current.scrollTop -= contentRef.current.clientHeight * 0.8;
    }
  };

  // 下一页
  const nextPage = () => {
    if (contentRef.current) {
      contentRef.current.scrollTop += contentRef.current.clientHeight * 0.8;
    }
  };

  if (loading) {
    return <div className="reader">加载中...</div>;
  }

  if (error) {
    return (
      <div className="reader">
        <div className="reader-error">{error}</div>
        <button onClick={goBack}>返回书架</button>
      </div>
    );
  }

  return (
    <div className={`reader ${theme}`}>
      <div className="reader-header">
        <button onClick={goBack}>返回书架</button>
        <h2>{filename}</h2>
        <div className="reader-controls">
          <button onClick={decreaseFontSize}>A-</button>
          <button onClick={increaseFontSize}>A+</button>
          <button onClick={toggleTheme}>
            {theme === "light" ? "夜间" : "日间"}
          </button>
        </div>
      </div>
      
      <div 
        className="reader-content"
        ref={contentRef}
        style={{ fontSize: `${fontSize}px` }}
      >
        <pre>{content}</pre>
      </div>
      
      <div className="reader-footer">
        <button onClick={prevPage}>上一页</button>
        <button onClick={nextPage}>下一页</button>
      </div>
    </div>
  );
};

export default Reader;