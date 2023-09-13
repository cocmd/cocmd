/* eslint-disable @typescript-eslint/explicit-function-return-type */
import { NamedSetState } from './middlewares/middlewares';
import { MyState } from './useStore';

export interface FilterSlice {
  filter: boolean | null;
  setFilter: (filerValue: boolean | null) => void;
}

const createFilterSlice = (
  set: NamedSetState<MyState>,
  get: NamedSetState<MyState>,
) => ({
  filter: null,
  setFilter: (filterValue: boolean | null) => {
    set(() => ({ filter: filterValue }), false, 'filter.setFilter');
  },
});

export default createFilterSlice;
