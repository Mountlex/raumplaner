import Cookies from "js-cookie";

export function getAuthorizationHeader() {
    const currentUser = Cookies.get("currentUser");

    console.log("token: " + currentUser)

    return {
      Authorization: "Bearer " + currentUser
    };
}