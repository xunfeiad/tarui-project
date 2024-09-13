import {get, post} from "../request";
export const login = async<T> (data: T) => post("/auth/login", data)