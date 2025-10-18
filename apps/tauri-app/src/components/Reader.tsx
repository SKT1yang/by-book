import React, { useState, useEffect, useRef } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { invoke } from "@tauri-apps/api/core";

interface PageContent {
  pageNumber: number;
  content: string;
}

// 新增：章节信息接口
interface ChapterInfo {
  title: string;
  startPage: number;
}

const Reader: React.FC = () => {
  const { filename } = useParams<{ filename: string }>();
  const navigate = useNavigate();
  const [rawContent, setRawContent] = useState("");
  const [pages, setPages] = useState<PageContent[]>([]);
  const [chapters, setChapters] = useState<ChapterInfo[]>([]); // 章节列表，包含起始页码
  const [currentChapter, setCurrentChapter] = useState(0); // 当前章节索引
  const [currentPage, setCurrentPage] = useState(0);
  const [totalPages, setTotalPages] = useState(0); // 整个文档的总页数
  const [loading, setLoading] = useState(true);
  const [loadingChapter, setLoadingChapter] = useState(false); // 专门用于章节加载的状态
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
      
      // 获取章节信息和页码映射
      const chapterList: string[] = await invoke<string[]>("parse_document_chapters", { content });
      
      // 初始化章节信息数组
      const chapterInfoList: ChapterInfo[] = chapterList.map((title, index) => ({
        title,
        startPage: 0 // 默认起始页码为0，后续会更新
      }));
      
      setChapters(chapterInfoList);
      
      // 获取章节与页码的映射关系
      const chapterPageMapping: [string, number][] = await invoke<[string, number][]>("get_document_chapter_page_mapping", { content });
      console.log("章节页码映射:", chapterPageMapping);
      
      // 更新章节信息
      const updatedChapters = chapterList.map((title, index) => ({
        title,
        startPage: chapterPageMapping[index]?.[1] || 0
      }));
      
      setChapters(updatedChapters);
      
      // 只加载第一章内容作为初始内容
      if (chapterInfoList.length > 0) {
        await loadChapterContent(content, 0);
      }
      
      setError("");
    } catch (err) {
      setError("加载文档失败: " + err);
    } finally {
      setLoading(false);
    }
  }

  /**
   * 加载特定章节内容
   * @param content - 文档内容
   * @param chapterIndex - 章节索引
   */
  async function loadChapterContent(content: string, chapterIndex: number) {
    try {
      console.log("=== 开始加载章节内容 ===");
      console.log("章节索引:", chapterIndex);
      setLoadingChapter(true);
      const result = await invoke<[string, number, number]>("load_chapter_content_with_offset", { 
        content, 
        chapterIndex 
      });
      
      const [typesetResult, startPage, totalPageCount] = result;
      console.log("排版结果:", typesetResult);
      console.log("章节起始页码:", startPage);
      console.log("文档总页数:", totalPageCount);
      
      processTypesetResult(typesetResult, startPage, totalPageCount);
      setCurrentChapter(chapterIndex);
      setCurrentPage(startPage); // 设置当前页为章节起始页
      setError("");
      console.log("=== 章节内容加载完成 ===");
    } catch (err) {
      console.error("加载章节失败:", err);
      setError("加载章节失败: " + err);
    } finally {
      setLoadingChapter(false);
    }
  }

  /**
   * 处理排版结果
   * @param result - 排版引擎返回的结果
   * @param startPage - 章节起始页码
   * @param totalPageCount - 文档总页数
   */
  function processTypesetResult(result: string, startPage: number, totalPageCount: number) {
    console.log("=== 开始处理排版结果 ===");
    console.log("章节起始页码:", startPage);
    console.log("文档总页数:", totalPageCount);
    
    // 解析排版引擎返回的结果
    const pageSections = result.split('\n\n').filter(section => section.trim() !== '');
    
    const processedPages: PageContent[] = [];
    
    for (const section of pageSections) {
      if (section.startsWith('Page ')) {
        // 提取页码和内容
        const lines = section.split('\n');
        const contentLines = lines.slice(2, -1); // 跳过页码行和开始标记行，去掉结束标记行
        const content = contentLines.join('\n');
        
        // 从排版结果中提取页码并加上起始页码偏移
        const pageNumberMatch = section.match(/^Page (\d+)/);
        const pageNumber = pageNumberMatch ? parseInt(pageNumberMatch[1]) : 1;
        
        processedPages.push({
          pageNumber: startPage + pageNumber - 1, // 调整为绝对页码
          content: content
        });
      }
    }
    
    console.log("处理后的页面:", processedPages);
    setPages(processedPages);
    setTotalPages(totalPageCount); // 更新总页数
    console.log("=== 排版结果处理完成 ===");
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
    console.log("=== 翻页操作开始 ===");
    console.log("点击上一页按钮，当前页:", currentPage);
    console.log("当前页面数据:", pages);
    console.log("总页数:", totalPages);
    if (currentPage > 0) {
      console.log("准备切换到上一页");
      setCurrentPage(prev => {
        const newPage = prev - 1;
        console.log("上一页，新页码:", newPage);
        return newPage;
      });
    } else {
      console.log("!!! 已在第一页，无法上翻");
      console.log("当前页码:", currentPage, "总页数:", totalPages);
    }
    console.log("=== 翻页操作结束 ===");
  };

  // 下一页
  const nextPage = () => {
    console.log("=== 翻页操作开始 ===");
    console.log("点击下一页按钮，当前页:", currentPage, "总页数:", totalPages);
    console.log("当前页面数据:", pages);
    if (currentPage < totalPages - 1) {
      console.log("准备切换到下一页");
      setCurrentPage(prev => {
        const newPage = prev + 1;
        console.log("下一页，新页码:", newPage);
        return newPage;
      });
    } else {
      console.log("!!! 已在最后一页，无法下翻");
      console.log("当前页码:", currentPage, "总页数:", totalPages);
    }
    console.log("=== 翻页操作结束 ===");
  };

  // 监听 currentPage 变化
  useEffect(() => {
    console.log("*** currentPage 状态更新为:", currentPage);
    console.log("*** 总页数:", totalPages);
    
    // 查找当前页属于哪个章节
    if (chapters.length > 0) {
      // 从后往前查找，找到第一个起始页码小于等于当前页码的章节
      let targetChapterIndex = 0;
      for (let i = chapters.length - 1; i >= 0; i--) {
        if (chapters[i].startPage <= currentPage) {
          targetChapterIndex = i;
          break;
        }
      }
      
      // 如果当前章节不是目标章节，则加载目标章节
      // 但只有在不是由章节切换引起的页码变化时才加载
      if (targetChapterIndex !== currentChapter) {
        console.log(`当前页${currentPage}属于章节${targetChapterIndex}，需要切换章节`);
        loadChapterContent(rawContent, targetChapterIndex);
      }
    }
  }, [currentPage, totalPages, chapters, currentChapter, rawContent]);

  // 监听 pages 变化
  useEffect(() => {
    console.log("*** pages 状态更新，新的页面数量:", pages.length);
    console.log("*** 页面数据:", pages);
    if (pages.length > 0) {
      console.log("*** 第一页内容预览:", pages[0].content.substring(0, 100) + "...");
    }
  }, [pages]);

  // 监听 currentChapter 变化
  useEffect(() => {
    console.log("*** currentChapter 状态更新为:", currentChapter);
  }, [currentChapter]);

  // 切换章节
  const handleChapterChange = async (e: React.ChangeEvent<HTMLSelectElement>) => {
    console.log("=== 章节切换开始 ===");
    console.log("切换到章节索引:", e.target.value);
    const chapterIndex = parseInt(e.target.value);
    
    // 加载新章节内容
    await loadChapterContent(rawContent, chapterIndex);
    console.log("=== 章节切换结束 ===");
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
                {chapter.title}
              </option>
            ))}
          </select>
        </div>
      )}
      
      <div 
        className="reader-content"
        ref={contentRef}
        style={{ fontSize: `${fontSize}px` }}
      >
        {(loadingChapter || loading) ? (
          <div>章节加载中...</div>
        ) : pages.length > 0 ? (
          <pre>{(() => {
            // 根据当前页码查找对应的页面内容
            const currentPageContent = pages.find(page => page.pageNumber === currentPage);
            console.log(">>> 渲染页面内容，当前页:", currentPage, "页面数据:", currentPageContent?.content);
            return currentPageContent?.content || "无内容";
          })()}</pre>
        ) : (
          <pre>{rawContent}</pre>
        )}
      </div>
      
      <div className="reader-footer">
        <button onClick={prevPage} disabled={currentPage === 0 || pages.length === 0 || loadingChapter}>
          上一页
        </button>
        <span>
          {pages.length > 0 ? `第 ${currentPage + 1} 页 / 共 ${totalPages} 页` : "无分页"}
        </span>
        <button 
          onClick={nextPage} 
          disabled={currentPage === totalPages - 1 || pages.length === 0 || loadingChapter}>
          下一页
        </button>
      </div>
    </div>
  );
};

export default Reader;