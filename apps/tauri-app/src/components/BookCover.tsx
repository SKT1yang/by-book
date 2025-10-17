import React from "react";

interface BookCoverProps {
  title: string;
  coverPath?: string;
  onClick?: () => void;
}

/**
 * 书籍封面组件
 * 
 * 显示书籍封面，如果没有封面则显示默认封面
 * 默认封面基于书名首字符生成，并配以个性化颜色
 */
const BookCover: React.FC<BookCoverProps> = ({ title, coverPath, onClick }) => {
  /**
   * 从书名提取首字母或汉字首字符作为默认封面文字
   * @param bookTitle 书籍标题
   * @returns 封面显示的文字
   */
  const getCoverText = (bookTitle: string) => {
    // 移除空白字符和特殊符号
    const cleanTitle = bookTitle.trim().replace(/[^\u4e00-\u9fa5a-zA-Z0-9]/g, '');
    
    if (cleanTitle.length === 0) return '书';
    
    // 如果是中文，返回第一个汉字
    const firstChar = cleanTitle.charAt(0);
    if (/[\u4e00-\u9fa5]/.test(firstChar)) {
      return firstChar;
    }
    
    // 如果是英文，返回首字母（大写）
    return firstChar.toUpperCase();
  };

  /**
   * 生成随机背景色，但保持一致性（相同书名始终是相同颜色）
   * @param bookTitle 书籍标题
   * @returns CSS颜色字符串
   */
  const getBackgroundColor = (bookTitle: string) => {
    // 基于书名生成哈希值
    let hash = 0;
    for (let i = 0; i < bookTitle.length; i++) {
      hash = bookTitle.charCodeAt(i) + ((hash << 5) - hash);
    }
    
    // 生成HSL颜色值，饱和度和亮度固定，只改变色相
    const hue = Math.abs(hash) % 360;
    return `hsl(${hue}, 70%, 45%)`;
  };

  const coverText = getCoverText(title);
  const backgroundColor = getBackgroundColor(title);

  if (coverPath) {
    // 如果有封面路径，显示实际封面
    return (
      <div 
        className="book-cover" 
        onClick={onClick}
        style={{ 
          backgroundImage: `url(${coverPath})`,
          backgroundSize: 'cover',
          backgroundPosition: 'center'
        }}
      >
        {!coverPath && (
          <div className="default-cover" style={{ backgroundColor }}>
            <span className="cover-text">{coverText}</span>
          </div>
        )}
      </div>
    );
  }

  // 显示默认封面
  return (
    <div 
      className="book-cover" 
      onClick={onClick}
    >
      <div className="default-cover" style={{ backgroundColor }}>
        <span className="cover-text">{coverText}</span>
      </div>
    </div>
  );
};

export default BookCover;