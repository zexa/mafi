import '../styles/globals.css'
import { Card, Text, NextUIProvider } from '@nextui-org/react';

import getConfig from 'next/config';
import cookie from 'js-cookie'
import { useState, useEffect, createContext } from 'react'

export const UserContext = createContext()

// export async function getServerSideProps(context) {
//     const session = context.req.headers.MAFI_SESSION ?? null;

//     const config = getConfig();
//     const serverRuntimeConfig = config.serverRuntimeConfig ?? null;
//     if (serverRuntimeConfig === null) {
//         return {
//             props: {
//                 error: {
//                     message: "Frontend server is misconfigured. Missing serverRuntimeConfig",
//                 }
//             },
//         };
//     }
//     const apiUrl = serverRuntimeConfig.apiUrl ?? null;
//     if (apiUrl === null) {
//         return {
//             props: {
//                 error: {
//                     message: "Frontend server is misconfigured. Missing apiUrl",
//                     config: config,
//                 }
//             },
//         };
//     }

//     const response = await fetch(`${apiUrl}/roles`);
//     if (!response.ok) {
//         return {
//             props: {
//                 error: {
//                     message: "Backend server error. Got response.",
//                 }
//             },
//         };
//     }
//     const roles = await response.json();

//     return { 
//         props: { 
//             session: session,
//             roles: roles,
//             error: "test",
//         } 
//     }
// }

const App = ({ Component, pageProps }) => {
  const [user, setUser] = useState()
  const [token, setToken] = useState()
  const cookieToken = cookie.get('token')
  const login = ({ user, token }) => {
    cookie.set('token', token, { expires: 14 })
    cookie.set('userId', user.id, { expires: 14 })
    setUser(user)
    setToken(token)
    trigger(`/api/users/${user.id}`)
  }
  const logout = () => {
    setUser(null)
    cookie.remove(token)
    // invalidate the user with swr
    mutate(`/api/users/${user.id}`, {}, false)
    Router.push('/login')
  }
  useEffect(() => {
    if (cookieToken) setToken(cookieToken)
  }, []);

  return (
    <UserContext.Provider
      value={{
        user: user,
        token: cookieToken || token,
        login: login,
        logout: logout,
        setUser: setUser,
    }}>
      <NextUIProvider>
        <Component {...pageProps} />
      </NextUIProvider>
    </UserContext.Provider>
  );
}

export default App
