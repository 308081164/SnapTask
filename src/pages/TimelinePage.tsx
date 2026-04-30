import React, { useEffect, useState } from 'react';
import { useTaskStore } from '@/stores/taskStore';
import { TaskDetail } from '@/components/TaskDetail/TaskDetail';
import { Timeline } from '@/components/Timeline/Timeline';
import type { Task } from '@/types';

const TimelinePage: React.FC = () => {
  const { tasks, fetchTasks } = useTaskStore();
  const [selectedTask, setSelectedTask] = useState<Task | null>(null);

  useEffect(() => {
    fetchTasks();
  }, [fetchTasks]);

  return (
    <>
      <Timeline tasks={tasks} onTaskClick={setSelectedTask} />
      <TaskDetail
        task={selectedTask}
        isOpen={!!selectedTask}
        onClose={() => setSelectedTask(null)}
        onUpdated={() => fetchTasks()}
      />
    </>
  );
};

export default TimelinePage;
