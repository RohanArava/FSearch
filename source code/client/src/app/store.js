import { configureStore } from '@reduxjs/toolkit';

const initial_state = {
  object: {
    time:0,
    query:"",
    results:[]
  }
}

const result_reducer = (state=initial_state, action)=>{
  switch(action.type){
    case "CHANGE_STATE":
      return {
        ...state,
        object: action.payload
      };
      default: return state;
  }
}

export const replaceObject = (newObj) =>{
  return {
    type: "CHANGE_STATE",
    payload: newObj
  }
}

export const store = configureStore({
  reducer: {
    objReducer: result_reducer
  },
});
