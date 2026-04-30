import React, { useState, useMemo, useEffect } from 'react';
import { Plus, Filter } from 'lucide-react';
import { TaskStatus, TaskPriority } from '@/types';
import type { Task } from '@/types';
import { useTaskStore } from '@/stores/taskStore';
import { useClientStore } from '@/stores/clientStore';
import { useProjectStore } from '@/stores/projectStore';
import { TaskColumn } from './TaskColumn';
import { TaskForm } from '@/components/TaskForm/TaskForm';
import { TaskDetail } from '@/components/TaskDetail/TaskDetail';
import { SearchBar } from '@/components/Search/SearchBar';
import { ScreenshotButton } from '@/components/ScreenshotButton/ScreenshotButton';
import { EmptyState } from '@/components/Common/EmptyState';
import { getPriorityOrder } from '@/utils/priority';
import styles from './TaskBoard.module.css';

export const TaskBoard: React.FC = () => {
  const { tasks, fetchTasks, updateTaskStatus, loading } = useTaskStore();
  const { clients, fetchClients } = useClientStore();
  const { projects, fetchProjects } = useProjectStore();

  const [selectedTask, setSelectedTask] = useState<Task | null>(null);
  const [showForm, setShowForm] = useState(false);
  const [showFilters, setShowFilters] = useState(false);
  const [filterPriority, setFilterPriority] = useState<TaskPriority | null>(null);
  const [filterClient, setFilterClient] = useState<string | null>(null);
  const [filterProject, setFilterProject] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState('');

  useEffect(() => {
    fetchTasks();
    fetchClients();
    fetchProjects();
  }, [fetchTasks, fetchClients, fetchProjects]);

  const getClientName = (clientId: string): string => {
    return clients.find((c) => c.id === clientId)?.name || '未知客户';
  };

  const getProjectName = (projectId: string): string => {
    return projects.find((p) => p.id === projectId)?.name || '未知项目';
  };

  const filteredTasks = useMemo(() => {
    let result = [...tasks];

    if (searchQuery) {
      const q = searchQuery.toLowerCase();
      result = result.filter(
        (t) =>
          t.title.toLowerCase().includes(q) ||
          t.description.toLowerCase().includes(q) ||
          t.tags.some((tag) => tag.toLowerCase().includes(q))
      );
    }

    if (filterPriority) {
      result = result.filter((t) => t.priority === filterPriority);
    }

    if (filterClient) {
      result = result.filter((t) => t.client_id === filterClient);
    }

    if (filterProject) {
      result = result.filter((t) => t.project_id === filterProject);
    }

    // Sort by priority (high first), then by deadline
    result.sort((a, b) => {
      const priorityDiff = getPriorityOrder(b.priority) - getPriorityOrder(a.priority);
      if (priorityDiff !== 0) return priorityDiff;
      if (a.deadline && b.deadline) {
        return new Date(a.deadline).getTime() - new Date(b.deadline).getTime();
      }
      if (a.deadline) return -1;
      if (b.deadline) return 1;
      return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
    });

    return result;
  }, [tasks, searchQuery, filterPriority, filterClient, filterProject]);

  const todoTasks = filteredTasks.filter((t) => t.status === TaskStatus.Todo);
  const inProgressTasks = filteredTasks.filter((t) => t.status === TaskStatus.InProgress);
  const doneTasks = filteredTasks.filter((t) => t.status === TaskStatus.Done);

  const handleDrop = (taskId: string, newStatus: TaskStatus) => {
    updateTaskStatus(taskId, newStatus);
  };

  const handleStatusChange = (taskId: string, newStatus: TaskStatus) => {
    updateTaskStatus(taskId, newStatus);
  };

  const handleTaskClick = (task: Task) => {
    setSelectedTask(task);
  };

  if (loading && tasks.length === 0) {
    return <EmptyState title="加载中..." description="正在获取任务列表" />;
  }

  return (
    <div className={styles.board}>
      {/* Header */}
      <div className={styles.header}>
        <div className={styles.headerLeft}>
          <h1 className={styles.pageTitle}>任务看板</h1>
          <span className={styles.taskCount}>{filteredTasks.length} 个任务</span>
        </div>
        <div className={styles.headerRight}>
          <SearchBar value={searchQuery} onChange={setSearchQuery} />
          <button
            className={`${styles.filterBtn} ${showFilters ? styles.filterActive : ''}`}
            onClick={() => setShowFilters(!showFilters)}
          >
            <Filter size={16} />
            筛选
          </button>
          <button className={styles.addBtn} onClick={() => setShowForm(true)}>
            <Plus size={16} />
            新建任务
          </button>
        </div>
      </div>

      {/* Filters */}
      {showFilters && (
        <div className={styles.filters}>
          <div className={styles.filterGroup}>
            <label className={styles.filterLabel}>优先级</label>
            <select
              className={styles.filterSelect}
              value={filterPriority || ''}
              onChange={(e) => setFilterPriority(e.target.value as TaskPriority || null)}
            >
              <option value="">全部</option>
              <option value={TaskPriority.Urgent}>紧急</option>
              <option value={TaskPriority.High}>高</option>
              <option value={TaskPriority.Medium}>中</option>
              <option value={TaskPriority.Low}>低</option>
            </select>
          </div>
          <div className={styles.filterGroup}>
            <label className={styles.filterLabel}>客户</label>
            <select
              className={styles.filterSelect}
              value={filterClient || ''}
              onChange={(e) => setFilterClient(e.target.value || null)}
            >
              <option value="">全部</option>
              {clients.map((c) => (
                <option key={c.id} value={c.id}>
                  {c.name}
                </option>
              ))}
            </select>
          </div>
          <div className={styles.filterGroup}>
            <label className={styles.filterLabel}>项目</label>
            <select
              className={styles.filterSelect}
              value={filterProject || ''}
              onChange={(e) => setFilterProject(e.target.value || null)}
            >
              <option value="">全部</option>
              {projects.map((p) => (
                <option key={p.id} value={p.id}>
                  {p.name}
                </option>
              ))}
            </select>
          </div>
          <button
            className={styles.clearFilters}
            onClick={() => {
              setFilterPriority(null);
              setFilterClient(null);
              setFilterProject(null);
            }}
          >
            清除筛选
          </button>
        </div>
      )}

      {/* Columns */}
      <div className={styles.columns}>
        <TaskColumn
          title="待办"
          status={TaskStatus.Todo}
          tasks={todoTasks}
          onTaskClick={handleTaskClick}
          onStatusChange={handleStatusChange}
          onDrop={handleDrop}
          getClientName={getClientName}
          getProjectName={getProjectName}
        />
        <TaskColumn
          title="进行中"
          status={TaskStatus.InProgress}
          tasks={inProgressTasks}
          onTaskClick={handleTaskClick}
          onStatusChange={handleStatusChange}
          onDrop={handleDrop}
          getClientName={getClientName}
          getProjectName={getProjectName}
        />
        <TaskColumn
          title="已完成"
          status={TaskStatus.Done}
          tasks={doneTasks}
          onTaskClick={handleTaskClick}
          onStatusChange={handleStatusChange}
          onDrop={handleDrop}
          getClientName={getClientName}
          getProjectName={getProjectName}
        />
      </div>

      {/* Task Detail Drawer */}
      <TaskDetail
        task={selectedTask}
        isOpen={!!selectedTask}
        onClose={() => setSelectedTask(null)}
        onUpdated={() => fetchTasks()}
      />

      {/* New Task Form */}
      <TaskForm
        isOpen={showForm}
        onClose={() => setShowForm(false)}
        onCreated={() => {
          setShowForm(false);
          fetchTasks();
        }}
      />

      {/* Screenshot FAB */}
      <ScreenshotButton />
    </div>
  );
};
