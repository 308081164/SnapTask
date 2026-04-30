import React, { useState } from 'react';
import { Camera, Loader2 } from 'lucide-react';
import { useAIStore } from '@/stores/aiStore';
import { screenshotApi } from '@/lib/tauri';
import { AnalysisConfirm } from '@/components/AnalysisConfirm/AnalysisConfirm';
import styles from './ScreenshotButton.module.css';

export const ScreenshotButton: React.FC = () => {
  const { analyzing, currentResult, analyzeScreenshot, confirmAnalysis } = useAIStore();
  const [showConfirm, setShowConfirm] = useState(false);

  const handleClick = async () => {
    try {
      const imageBase64 = await screenshotApi.triggerScreenshot();
      const result = await analyzeScreenshot(imageBase64);
      if (result.extracted_tasks.length > 0) {
        setShowConfirm(true);
      }
    } catch (error) {
      console.error('截屏分析失败:', error);
    }
  };

  const handleConfirm = async (result: typeof currentResult) => {
    if (!result) return;
    try {
      await confirmAnalysis(result);
      setShowConfirm(false);
    } catch (error) {
      console.error('确认分析结果失败:', error);
    }
  };

  const handleCancel = () => {
    setShowConfirm(false);
  };

  return (
    <>
      <button
        className={`${styles.fab} ${analyzing ? styles.analyzing : ''}`}
        onClick={handleClick}
        disabled={analyzing}
        title="截屏识别任务"
      >
        {analyzing ? (
          <Loader2 size={22} className={styles.spinning} />
        ) : (
          <Camera size={22} />
        )}
        {analyzing && <span className={styles.fabLabel}>分析中...</span>}
      </button>

      {showConfirm && currentResult && (
        <AnalysisConfirm
          result={currentResult}
          onConfirm={handleConfirm}
          onCancel={handleCancel}
        />
      )}
    </>
  );
};
