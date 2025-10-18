import React, { useState, useEffect, useRef } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { invoke } from "@tauri-apps/api/core";

interface PageContent {
  pageNumber: number;
  content: string;
}

const Reader: React.FC = () => {
  const { filename } = useParams<{ filename: string }>();
  const navigate = useNavigate();
  const [rawContent, setRawContent] = useState("");
  const [pages, setPages] = useState<PageContent[]>([]);
  const [chapters, setChapters] = useState<string[]>([]); // 章节列表
  const [chapterPageMapping, setChapterPageMapping] = useState<Map<string, number>>(new Map()); // 章节到页面的映射
  const [currentChapter, setCurrentChapter] = useState(0); // 当前章节索引
  const [currentPage, setCurrentPage] = useState(0);
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

  /**
   * 加载文档内容
   * @param filename - 要加载的文件名
   */
  async function loadDocument(filename: string) {
    try {
      setLoading(true);
      const content = await invoke<string>("load_document", { filename });
      setRawContent(content);
      
      // 获取章节信息
      const chapterList = await invoke<string[]>("parse_document_chapters", { content });
      setChapters(chapterList);
      
      // 使用排版引擎处理内容并获取章节页面映射
      const result = await invoke<[string, [string, number][]]>("typeset_document_with_chapter_info", { content });
      const [typesetResult, chapterPageMap] = result;
      
      // 转换章节页面映射为Map对象
      const chapterPageMapObj = new Map<string, number>();
      const chapterTitles: string[] = [];
      chapterPageMap.forEach(([title, page]) => {
        chapterPageMapObj.set(title, page);
        chapterTitles.push(title);
      });
      
      setChapterPageMapping(chapterPageMapObj);
      
      processTypesetResult(typesetResult);
      
      setError("");
    } catch (err) {
      setError("加载文档失败: " + err);
    } finally {
      setLoading(false);
    }
  }

  /**
   * 处理排版结果
   * @param result - 排版引擎返回的结果
   */
  function processTypesetResult(result: string) {
    // 解析排版引擎返回的结果
    const pageSections = result.split('\n\n').filter(section => section.trim() !== '');
    
    const processedPages: PageContent[] = [];
    let currentPageNumber = 1;
    
    for (const section of pageSections) {
      if (section.startsWith('Page ')) {
        // 提取页码和内容
        const lines = section.split('\n');
        const contentLines = lines.slice(2, -1); // 跳过页码行和开始标记行，去掉结束标记行
        const content = contentLines.join('\n');
        
        processedPages.push({
          pageNumber: currentPageNumber++,
          content: content
        });
      }
    }
    
    setPages(processedPages);
    setCurrentPage(0); // 重置到第一页
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
    if (currentPage > 0) {
      setCurrentPage(prev => prev - 1);
    }
  };

  // 下一页
  const nextPage = () => {
    if (currentPage < pages.length - 1) {
      setCurrentPage(prev => prev + 1);
    }
  };

  // 切换章节
  const handleChapterChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const chapterIndex = parseInt(e.target.value);
    setCurrentChapter(chapterIndex);
    
    // 获取章节标题
    const chapterTitle = chapters[chapterIndex];
    
    // 根据章节页面映射跳转到对应的页面
    if (chapterPageMapping.has(chapterTitle)) {
      const pageNumber = chapterPageMapping.get(chapterTitle)!;
      // 确保页码在有效范围内
      const validPageNumber = Math.min(Math.max(0, pageNumber), pages.length - 1);
      setCurrentPage(validPageNumber);
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
      
      {/* 章节导航栏 */}
      {chapters.length > 0 && (
        <div className="chapter-navigation">
          <select value={currentChapter} onChange={handleChapterChange}>
            {chapters.map((chapter, index) => (
              <option key={index} value={index}>
                {chapter}
              </option>
            ))}
          </select>
        </div>
      )}
      
      {pages.length > 0 ? (
        <div 
          className="reader-content"
          ref={contentRef}
          style={{ fontSize: `${fontSize}px` }}
        >
          <pre>{pages[currentPage]?.content || "无内容"}</pre>
        </div>
      ) : (
        <div className="reader-content" style={{ fontSize: `${fontSize}px` }}>
          <pre>{rawContent}</pre>
        </div>
      )}
      
      <div className="reader-footer">
        <button onClick={prevPage} disabled={currentPage === 0 || pages.length === 0}>
          上一页
        </button>
        <span>
          {pages.length > 0 ? `第 ${currentPage + 1} 页 / 共 ${pages.length} 页` : "无分页"}
        </span>
        <button 
          onClick={nextPage} 
          disabled={currentPage === pages.length - 1 || pages.length === 0}
        >
          下一页
        </button>
      </div>
    </div>
  );
};

export default Reader;