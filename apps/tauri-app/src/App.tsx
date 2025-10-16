import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-dialog';
import { readTextFile } from '@tauri-apps/plugin-fs';
import "./App.css";

function App() {
  const [documentContent, setDocumentContent] = useState("");
  const [typesetResult, setTypesetResult] = useState("");
  const [documents, setDocuments] = useState<string[]>([]);
  const [selectedDocument, setSelectedDocument] = useState("");
  const [errorMessage, setErrorMessage] = useState("");

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
        // 读取文件内容
        const content = await readTextFile(selected as string);
        setDocumentContent(content);
        
        // 保存文件到应用数据目录
        const fileName = (selected as string).split('/').pop() || (selected as string).split('\\').pop() || "uploaded.txt";
        await invoke("save_document", { filename: fileName, content: content });
        
        // 刷新文档列表
        await loadDocuments();
        setSelectedDocument(fileName);
      }
    } catch (error) {
      console.error("上传文件失败:", error);
      setErrorMessage("上传文件失败: " + error);
    }
  }

  // 加载选中的文档
  async function loadSelectedDocument() {
    if (!selectedDocument) {
      setErrorMessage("请选择一个文档");
      return;
    }
    
    try {
      setErrorMessage(""); // 清除之前的错误信息
      const content = await invoke<string>("load_document", { filename: selectedDocument });
      setDocumentContent(content);
    } catch (error) {
      console.error("加载文档失败:", error);
      setErrorMessage("加载文档失败: " + error);
    }
  }

  // 排版文档
  async function typeset() {
    try {
      setErrorMessage(""); // 清除之前的错误信息
      const result = await invoke("typeset_document", { content: documentContent });
      setTypesetResult(result as string);
    } catch (error) {
      setTypesetResult("Error: " + error);
      setErrorMessage("排版文档失败: " + error);
    }
  }

  return (
    <main className="container">
      <h1>排版引擎测试</h1>

      <div>
        {/* 错误信息显示区域 */}
        {errorMessage && (
          <div style={{ color: 'red', marginBottom: '10px' }}>
            错误: {errorMessage}
          </div>
        )}

        <div style={{ marginBottom: '10px' }}>
          <button onClick={uploadFile}>上传TXT文件</button>
        </div>

        <div style={{ marginBottom: '10px' }}>
          <select 
            value={selectedDocument} 
            onChange={(e) => setSelectedDocument(e.target.value)}
            style={{ marginRight: '10px' }}
          >
            <option value="">选择文档</option>
            {documents.map((doc) => (
              <option key={doc} value={doc}>{doc}</option>
            ))}
          </select>
          <button onClick={loadSelectedDocument}>加载文档</button>
        </div>

        <textarea
          value={documentContent}
          onChange={(e) => setDocumentContent(e.target.value)}
          placeholder="在此输入文档内容进行排版测试"
          rows={10}
          cols={50}
        />
        <br />
        <button onClick={typeset}>排版文档</button>
        <div style={{ marginTop: '10px' }}>
          <h3>排版结果：</h3>
          <pre style={{ textAlign: 'left', whiteSpace: 'pre-wrap' }}>{typesetResult}</pre>
        </div>
      </div>
    </main>
  );
}

export default App;