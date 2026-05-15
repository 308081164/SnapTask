import React, { useState, useEffect, useCallback, useRef } from 'react';
import { getCurrent } from '@tauri-apps/api/window';
import styles from './ScreenshotSelect.module.css';

interface Position {
  x: number;
  y: number;
}

const ScreenshotSelect: React.FC = () => {
  const [isDrawing, setIsDrawing] = useState(false);
  const [startPos, setStartPos] = useState<Position | null>(null);
  const [endPos, setEndPos] = useState<Position | null>(null);
  const [screenSize, setScreenSize] = useState({ width: 1920, height: 1080 });
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleResize = () => {
      if (containerRef.current) {
        setScreenSize({
          width: window.innerWidth,
          height: window.innerHeight,
        });
      }
    };
    handleResize();
    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  }, []);

  const getAreaRect = useCallback(() => {
    if (!startPos || !endPos) return null;
    return {
      x: Math.min(startPos.x, endPos.x),
      y: Math.min(startPos.y, endPos.y),
      width: Math.abs(endPos.x - startPos.x),
      height: Math.abs(endPos.y - startPos.y),
    };
  }, [startPos, endPos]);

  const handleMouseDown = useCallback((e: React.MouseEvent) => {
    setIsDrawing(true);
    setStartPos({ x: e.clientX, y: e.clientY });
    setEndPos({ x: e.clientX, y: e.clientY });
  }, []);

  const handleMouseMove = useCallback((e: React.MouseEvent) => {
    if (!isDrawing) return;
    setEndPos({ x: e.clientX, y: e.clientY });
  }, [isDrawing]);

  const handleMouseUp = useCallback(async () => {
    setIsDrawing(false);
    const rect = getAreaRect();
    if (rect && rect.width > 10 && rect.height > 10) {
      window.api.screenshot.captureArea(rect.x, rect.y, rect.width, rect.height);
      await getCurrent().hide();
    }
    setStartPos(null);
    setEndPos(null);
  }, [getAreaRect]);

  const handleKeyDown = useCallback(async (e: React.KeyboardEvent) => {
    if (e.key === 'Escape') {
      await getCurrent().hide();
      setStartPos(null);
      setEndPos(null);
    } else if (e.key === 'Enter' && !isDrawing) {
      const rect = getAreaRect();
      if (rect && rect.width > 10 && rect.height > 10) {
        window.api.screenshot.captureArea(rect.x, rect.y, rect.width, rect.height);
        await getCurrent().hide();
      }
    }
  }, [getAreaRect, isDrawing]);

  const rect = getAreaRect();

  return (
    <div
      ref={containerRef}
      className={styles.container}
      onMouseDown={handleMouseDown}
      onMouseMove={handleMouseMove}
      onMouseUp={handleMouseUp}
      onMouseLeave={handleMouseUp}
      onKeyDown={handleKeyDown}
      tabIndex={0}
    >
      {/* 遮罩层 */}
      <div className={styles.overlay} />
      
      {/* 选择区域 */}
      {rect && rect.width > 0 && rect.height > 0 && (
        <div
          className={styles.selection}
          style={{
            left: rect.x,
            top: rect.y,
            width: rect.width,
            height: rect.height,
          }}
        >
          {/* 边角标记 */}
          <div className={styles.corner topLeft} />
          <div className={styles.corner topRight} />
          <div className={styles.corner bottomLeft} />
          <div className={styles.corner bottomRight} />
          
          {/* 尺寸显示 */}
          <div className={styles.dimensions}>
            {rect.width} × {rect.height}
          </div>
        </div>
      )}

      {/* 操作提示 */}
      <div className={styles.hints}>
        <div className={styles.hint}>
          <span className={styles.key}>拖拽</span> 选择区域
        </div>
        <div className={styles.hint}>
          <span className={styles.key}>Enter</span> 确认截图
        </div>
        <div className={styles.hint}>
          <span className={styles.key}>Esc</span> 取消
        </div>
      </div>
    </div>
  );
};

export default ScreenshotSelect;
