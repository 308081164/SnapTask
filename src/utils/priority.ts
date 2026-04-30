import { TaskPriority } from '@/types';

/**
 * 获取优先级对应的颜色
 */
export function getPriorityColor(priority: TaskPriority): string {
  switch (priority) {
    case TaskPriority.Urgent:
      return '#ef4444'; // red
    case TaskPriority.High:
      return '#f97316'; // orange
    case TaskPriority.Medium:
      return '#eab308'; // yellow
    case TaskPriority.Low:
      return '#22c55e'; // green
    default:
      return '#6b7280'; // gray
  }
}

/**
 * 获取优先级对应的背景色（浅色）
 */
export function getPriorityBgColor(priority: TaskPriority): string {
  switch (priority) {
    case TaskPriority.Urgent:
      return 'rgba(239, 68, 68, 0.15)';
    case TaskPriority.High:
      return 'rgba(249, 115, 22, 0.15)';
    case TaskPriority.Medium:
      return 'rgba(234, 179, 8, 0.15)';
    case TaskPriority.Low:
      return 'rgba(34, 197, 94, 0.15)';
    default:
      return 'rgba(107, 114, 128, 0.15)';
  }
}

/**
 * 获取优先级标签文字
 */
export function getPriorityLabel(priority: TaskPriority): string {
  switch (priority) {
    case TaskPriority.Urgent:
      return '紧急';
    case TaskPriority.High:
      return '高';
    case TaskPriority.Medium:
      return '中';
    case TaskPriority.Low:
      return '低';
    default:
      return '未知';
  }
}

/**
 * 获取优先级排序值（数值越大优先级越高）
 */
export function getPriorityOrder(priority: TaskPriority): number {
  switch (priority) {
    case TaskPriority.Urgent:
      return 4;
    case TaskPriority.High:
      return 3;
    case TaskPriority.Medium:
      return 2;
    case TaskPriority.Low:
      return 1;
    default:
      return 0;
  }
}
