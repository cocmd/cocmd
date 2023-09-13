import { Todo } from '../../model/Todo';
import { useStore } from '../useStore';

export const useTodosSelector = (): Todo[] => {
  const todos = useStore(({ filter, todos }) => {
    if (filter === null) {
      return todos;
    }
    return todos.filter(({ completed }) => completed === filter);
  });

  return todos;
};
