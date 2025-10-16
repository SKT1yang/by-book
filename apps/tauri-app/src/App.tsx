import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [documentContent, setDocumentContent] = useState("# 第一章\n\n这是测试内容。\n\n## 小节\n\n这是小节内容。\n\n# 第二章\n\n这是第二章的内容，用来测试分页功能。\n\n更多的内容...\n\n还有更多内容...\n\n这应该足以生成至少两页内容。\n\n继续添加更多内容...\n\n还需要更多内容才能看到分页效果。\n\n再加一点内容。\n\n差不多够了。");
  const [typesetResult, setTypesetResult] = useState("");

  async function typeset() {
    try {
      const result = await invoke("typeset_document", { content: documentContent });
      setTypesetResult(result as string);
    } catch (error) {
      setTypesetResult("Error: " + error);
    }
  }

  return (
    <main className="container">
      <h1>排版引擎测试</h1>

      <div>
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
