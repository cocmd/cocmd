import { PartialState, State } from 'zustand';

export type NamedSetState<T extends State> = (
  partial: PartialState<T>,
  replace?: boolean,
  name?: string
) => void;
