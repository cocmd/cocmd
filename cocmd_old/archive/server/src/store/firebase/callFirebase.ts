/* eslint-disable object-curly-newline */
/* eslint-disable max-len */
/* eslint-disable no-promise-executor-return */
/* eslint-disable prefer-promise-reject-errors */

// DECOMMENTER A PARTIR DICI QUAND FIREBASE EST BRANCHEE

// import { collection, deleteDoc, doc, getDocs, setDoc, updateDoc } from 'firebase/firestore';

// import { db } from '../../config/firebaseConfig';
// import { Todo } from '../../model/Todo';

// export const getTodosFromFirebase = async (): Promise<Todo[]> => {
//   const todosCollectionRef = collection(db, 'todos');
//   const data = await getDocs(todosCollectionRef);
//   const res: Todo[] = data.docs.map((doc) => doc.data() as Todo);
//   return res;
// };

// export const addTodoToFirebase = async (todo: Todo): Promise<Todo> => {
//   const { id } = todo;
//   try {
//     await setDoc(doc(db, 'todos', id), {
//       ...todo,
//     });
//     return todo;
//   } catch (err) {
//     return Promise.reject(new Error('fail'));
//   }
// };

// export const deleteTodoFromFirebase = async (
//   deleteId: string,
// ): Promise<string> => {
//   try {
//     await deleteDoc(doc(db, 'todos', deleteId));
//     return deleteId;
//   } catch (err) {
//     return Promise.reject(new Error('fail'));
//   }
// };

// export const toggleCompleteTodoToFirebase = async (
//   todo: Todo,
// ): Promise<Todo> => {
//   const { id: toggledTodoId, completed } = todo;
//   const todoDocRef = doc(db, 'todos', toggledTodoId);

//   try {
//     await updateDoc(todoDocRef, {
//       ...todo,
//       completed: !completed,
//     });
//     return todo;
//   } catch (error) {
//     return Promise.reject(new Error('fail'));
//   }
// };

// JUSQUI ICI

// delete cet export, juste là pour permettre la compilation de ce fichier en attendais que firebase soit rempli
export const toDelete = 'delete this';
