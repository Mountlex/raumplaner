import { NextResponse } from "next/server";

export const protectedRoutes = ["/rooms"];
export const authRoutes = ["/login"];
export const publicRoutes = ["/"];

export function middleware(request) {
  const token = request.cookies.get("token")?.value;
  
  if (
    protectedRoutes.includes(request.nextUrl.pathname) && !token
  ) {
    request.cookies.delete("token");
    const response = NextResponse.redirect(new URL("/login", request.url));
    response.cookies.delete("token");

    return response;
  } else {
    // Clone the request headers and set a new header `x-hello-from-middleware1`
  const requestHeaders = new Headers(request.headers)
  requestHeaders.set('authorization', 'Bearer ' + token)

  const response = NextResponse.next({
    request: {
      headers: requestHeaders,
    },
  })

  return response
  }
  

//   if (authRoutes.includes(request.nextUrl.pathname) && currentUser) {
//     return NextResponse.redirect(new URL("/profile", request.url));
//   }
}