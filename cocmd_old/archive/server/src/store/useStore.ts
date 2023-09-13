import create from 'zustand';
import { devtools } from 'zustand/middleware';

import createFilterSlice, { FilterSlice } from './createFilterSlice';
import createTodoSlice, { TodoSlice } from './createTodoSlice';

export type MyState = TodoSlice & FilterSlice;

export const useStore = create<MyState>()(
  devtools((set, get) => ({
    ...createTodoSlice(set, get),
    ...createFilterSlice(set, get),
  })),
);
