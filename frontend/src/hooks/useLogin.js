import Cookies from "js-cookie";
import { authService } from "../services";

export const useLogin = () => {
    const login = async (username, password) => {
      const user = await authService.login(username, password);
      console.log(user)
      if (user) {
        Cookies.set("currentUser", JSON.stringify(user));
      }
      return user;
    };
  
    return { login };
  };