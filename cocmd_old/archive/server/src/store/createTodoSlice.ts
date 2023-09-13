/* eslint-disable @typescript-eslint/explicit-function-return-type */
/* eslint-disable @typescript-eslint/no-unused-vars */
/* eslint-disable max-len */
import { Todo } from '../model/Todo';
import { NamedSetState } from './middlewares/middlewares';
import { MyState } from './useStore.js';

// import { deleteTodoFromFirebase, getTodosFromFirebase, toggleCompleteTodoToFirebase } from './firebase/callFirebase';
export interface TodoSlice {
  todos: Todo[];
  addTodo: (newTodo: Todo) => void;
  deleteTodo: (deleteId: string) => void;
  getTodos: () => void;
  toggleComplete: (toggledTodo: Todo) => void;
}

const createTodoSlice = (
  set: NamedSetState<MyState>,
  get: NamedSetState<MyState>,
) => ({
  todos: [],
  addTodo: async (newTodo: Todo) => {
    // DECOMMENTER UNE FOIS FIREBASE LINK
    // await addTodoToFirebase(newTodo);
    set(({ todos }) => ({ todos: [...todos, newTodo] }), false, 'todos.addTodo');
  },
  deleteTodo: async (deleteId: string) => {
    // DECOMMENTER UNE FOIS FIREBASE LINK
    // await deleteTodoFromFirebase(deleteId);
    set(
      ({ todos }) => ({
        todos: todos.filter(({ id }) => id !== deleteId),
      }),
      false,
      'todos.deleteTodo',
    );
  },
  getTodos: async () => {
    // DECOMMENTER UNE FOIS FIREBASE LINK
    // const res = await getTodosFromFirebase();
    // set({ todos: res }, false, 'todos.getTodos');

    // SUPPRIMER OU COMMENTER UNE FOIS FIREBASE LINK
    const arrayEmulateFirebase = [
      {
        id: 'ezaef',
        description: 'delete todo after link firebase',
        completed: false,
      },
    ];

    set({
      todos: arrayEmulateFirebase,
    }, false, 'todos.getTodos');
  },
  toggleComplete: async (updateTodo: Todo) => {
    // DECOMMENTER UNE FOIS FIREBASE LINK

    // await toggleCompleteTodoToFirebase(updateTodo);
    // set(
    //   ({ todos }) => ({
    //     todos: todos.map((todo) => (todo.id === toggledTodo.id
    //       ? { ...todo, completed: !toggledTodo.completed }
    //       : todo)),
    //   }),
    //   false,
    //   'todos.toggleComplete',
    // );

    // SUPPRIMER OU COMMENTER UNE FOIS FIREBASE LINK
    set(
      ({ todos }) => ({
        todos: todos.map((todo) => (todo.id === updateTodo.id
          ? { ...todo, completed: !updateTodo.completed }
          : todo)),
      }),
      false,
      'todos.toggleComplete',
    );
  },
});

export default createTodoSlice;
