import { format, formatDistanceToNow, differenceInDays, isPast, parseISO } from 'date-fns';
import { zhCN } from 'date-fns/locale';

/**
 * 格式化日期为指定格式
 */
export function formatDate(dateStr: string | null | undefined, fmt: string = 'yyyy-MM-dd HH:mm'): string {
  if (!dateStr) return '--';
  try {
    return format(parseISO(dateStr), fmt, { locale: zhCN });
  } catch {
    return '--';
  }
}

/**
 * 格式化为相对时间（如"3天前"、"2小时后"）
 */
export function formatRelative(dateStr: string | null | undefined): string {
  if (!dateStr) return '--';
  try {
    return formatDistanceToNow(parseISO(dateStr), { addSuffix: true, locale: zhCN });
  } catch {
    return '--';
  }
}

/**
 * 获取距离某日期还有多少天
 */
export function getDaysUntil(dateStr: string | null | undefined): number | null {
  if (!dateStr) return null;
  try {
    return differenceInDays(parseISO(dateStr), new Date());
  } catch {
    return null;
  }
}

/**
 * 判断是否已过期
 */
export function isOverdue(dateStr: string | null | undefined): boolean {
  if (!dateStr) return false;
  try {
    return isPast(parseISO(dateStr));
  } catch {
    return false;
  }
}

/**
 * 解析截止日期字符串为 Date 对象
 */
export function parseDeadline(dateStr: string | null | undefined): Date | null {
  if (!dateStr) return null;
  try {
    return parseISO(dateStr);
  } catch {
    return null;
  }
}

/**
 * 格式化截止日期的倒计时描述
 */
export function formatDeadlineCountdown(dateStr: string | null | undefined): string {
  const days = getDaysUntil(dateStr);
  if (days === null) return '';
  if (days < 0) return `已逾期 ${Math.abs(days)} 天`;
  if (days === 0) return '今天截止';
  if (days === 1) return '明天截止';
  return `${days} 天后截止`;
}

/**
 * 获取日期所属的月份标题
 */
export function formatMonthTitle(date: Date): string {
  return format(date, 'yyyy年M月', { locale: zhCN });
}

/**
 * 获取日期所属的星期
 */
export function formatWeekday(date: Date): string {
  return format(date, 'EEEE', { locale: zhCN });
}

/**
 * 判断两个日期是否同一天
 */
export function isSameDay(a: Date, b: Date): boolean {
  return (
    a.getFullYear() === b.getFullYear() &&
    a.getMonth() === b.getMonth() &&
    a.getDate() === b.getDate()
  );
}
