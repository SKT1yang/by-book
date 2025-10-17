import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useNavigate } from "react-router-dom";
import { open } from '@tauri-apps/plugin-dialog';
import { readTextFile } from '@tauri-apps/plugin-fs';
import BookCover from "./BookCover";

const Bookshelf: React.FC = () => {
  const [documents, setDocuments] = useState<string[]>([]);
  const [errorMessage, setErrorMessage] = useState("");
  const navigate = useNavigate();

  // 组件加载时获取文档列表
  useEffect(() => {
    loadDocuments();
  }, []);

  // 获取文档列表
  async function loadDocuments() {
    try {
      const docs = await invoke<string[]>("list_documents");
      setDocuments(docs);
      setErrorMessage(""); // 清除错误信息
    } catch (error) {
      console.error("获取文档列表失败:", error);
      setErrorMessage("获取文档列表失败: " + error);
    }
  }

  // 上传文件
  async function uploadFile() {
    try {
      setErrorMessage(""); // 清除之前的错误信息
      
      const selected = await open({
        multiple: false,
        filters: [{
          name: "Text Files",
          extensions: ["txt"]
        }]
      });

      if (selected) {
        // 读取文件内容，Tauri的readTextFile函数默认使用UTF-8编码
        const content = await readTextFile(selected as string);
        
        // 提取文件名
        const pathParts = (selected as string).split(/[/\\]/);
        const fileName = pathParts[pathParts.length - 1] || "uploaded.txt";
        
        console.log("准备保存文件:", fileName);
        const result = await invoke("save_document", { filename: fileName, content: content });
        console.log("保存文件结果:", result);
        
        // 刷新文档列表
        await loadDocuments();
      }
    } catch (error) {
      console.error("上传文件失败:", error);
      setErrorMessage("上传文件失败: " + error);
    }
  }

  // 打开阅读器
  const openReader = (filename: string) => {
    navigate(`/reader/${filename}`);
  };

  return (
    <div className="bookshelf">
      <h1>书架</h1>
      
      {/* 错误信息显示区域 */}
      {errorMessage && (
        <div className="error-message">
          错误: {errorMessage}
        </div>
      )}

      <div className="bookshelf-actions">
        <button onClick={uploadFile}>添加书籍</button>
      </div>

      <div className="book-list">
        {documents.length === 0 ? (
          <p>书架暂无书籍，请添加书籍</p>
        ) : (
          documents.map((doc) => (
            <div key={doc} className="book-item">
              <BookCover 
                title={doc} 
                onClick={() => openReader(doc)} 
              />
              <div className="book-title">{doc}</div>
            </div>
          ))
        )}
      </div>
    </div>
  );
};

export default Bookshelf;