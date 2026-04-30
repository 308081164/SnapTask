import React, { useEffect } from 'react';
import { useTaskStore } from '@/stores/taskStore';
import { useClientStore } from '@/stores/clientStore';
import { useProjectStore } from '@/stores/projectStore';
import { TaskBoard } from '@/components/TaskBoard/TaskBoard';

const TaskBoardPage: React.FC = () => {
  const { fetchTasks } = useTaskStore();
  const { fetchClients } = useClientStore();
  const { fetchProjects } = useProjectStore();

  useEffect(() => {
    fetchTasks();
    fetchClients();
    fetchProjects();
  }, [fetchTasks, fetchClients, fetchProjects]);

  return <TaskBoard />;
};

export default TaskBoardPage;
