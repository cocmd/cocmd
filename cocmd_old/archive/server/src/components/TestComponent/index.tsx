/* eslint-disable import/extensions */
/* eslint-disable object-curly-newline */
// eslint-disable-next-line object-curly-newline
import { nanoid } from 'nanoid';
import { ChangeEvent, MouseEvent, ReactElement, useEffect, useState } from 'react';

import { useTodosSelector } from '../../store/selectors/useTodoSelector';
import { useStore } from '../../store/useStore';
import { Content, Input, Wrapper } from './testComponent.tw';

const TestComponent = (): ReactElement => {
  const [todoTitle, setTodoTitle] = useState('');
  const { addTodo, deleteTodo, getTodos, toggleComplete, filter, setFilter } = useStore();

  useEffect(() => {
    getTodos();
  }, []);

  const todos = useTodosSelector();

  const handleAddTodo = (e: MouseEvent<HTMLButtonElement>): void => {
    e.preventDefault();
    if (todoTitle.length) {
      const newTodo = {
        id: nanoid(),
        description: todoTitle,
        completed: false,
      };
      addTodo(newTodo);
      setTodoTitle('');
    }
  };

  return (
    <Wrapper>
      <Content>
        <form className="flex">
          <Input
            type="text"
            name="todoTitle"
            value={todoTitle}
            placeholder="new todo"
            onChange={
              (e: ChangeEvent<HTMLInputElement>) => setTodoTitle(e.target.value)
            }
          />
          <button
            type="submit"
            className="default-btn bg-cyan-500"
            onClick={(e: MouseEvent<HTMLButtonElement>) => handleAddTodo(e)}
          >
            add
          </button>
        </form>
        <br />
        <ul>
          {todos?.map((todo) => (
            <li
              className={`${todo.completed ? 'line-through' : ''}`}
              key={todo.id}
            >
              {todo.description}
              <button
                className="default-btn bg-red-500"
                type="button"
                onClick={() => deleteTodo(todo.id)}
              >
                X
              </button>
              <button
                type="button"
                onClick={() => toggleComplete(todo)}
                className="default-btn bg-green-500"
              >
                {todo.completed ? 'undone' : 'done'}
              </button>
            </li>
          ))}
        </ul>
        <br />
        <div>
          <button
            disabled={filter === null}
            type="button"
            className="default-btn bg-blue-500 disabled:bg-red-400 text-white py-2 px-4"
            onClick={() => setFilter(null)}
          >
            All
          </button>
          <button
            disabled={filter === false}
            type="button"
            className="default-btn bg-blue-500 disabled:bg-red-400 text-white py-2 px-4"
            onClick={() => setFilter(false)}
          >
            To Do
          </button>
          <button
            disabled={filter === true}
            type="button"
            className="default-btn bg-blue-500 disabled:bg-red-400 text-white py-2 px-4"
            onClick={() => setFilter(true)}
          >
            Done
          </button>
        </div>
      </Content>
    </Wrapper>
  );
};

export default TestComponent;
