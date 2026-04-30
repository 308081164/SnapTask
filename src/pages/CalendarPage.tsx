import React, { useEffect, useState } from 'react';
import { useTaskStore } from '@/stores/taskStore';
import { TaskDetail } from '@/components/TaskDetail/TaskDetail';
import { CalendarView } from '@/components/Calendar/CalendarView';
import type { Task } from '@/types';

const CalendarPage: React.FC = () => {
  const { tasks, fetchTasks } = useTaskStore();
  const [selectedTask, setSelectedTask] = useState<Task | null>(null);

  useEffect(() => {
    fetchTasks();
  }, [fetchTasks]);

  return (
    <>
      <CalendarView tasks={tasks} onTaskClick={setSelectedTask} />
      <TaskDetail
        task={selectedTask}
        isOpen={!!selectedTask}
        onClose={() => setSelectedTask(null)}
        onUpdated={() => fetchTasks()}
      />
    </>
  );
};

export default CalendarPage;
