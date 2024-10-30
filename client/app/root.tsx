import {
  Links,
  Meta,
  Outlet,
  Scripts,
  ScrollRestoration,
} from "@remix-run/react";
import "./tailwind.css";
import {
  FluentProvider,
  IdPrefixProvider,
  webLightTheme,
} from "@fluentui/react-components";

export function Layout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <head>
        <meta charSet="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <Meta />
        <Links />
      </head>
      <body>
        <div className="flex flex-col h-screen gap-6">
          <div className="bg-slate-600 flex w-full h-[64px] py-[8px] px-[12px] shadow-md">
            <h1 className="font-bold text-1xl flex items-center justify-center text-white">
              Passkey-Ex
            </h1>
          </div>
          {children}
        </div>

        <ScrollRestoration />
        <Scripts />
      </body>
    </html>
  );
}

export default function App() {
  return (
    <IdPrefixProvider value="APPID-">
      <FluentProvider theme={webLightTheme}>
        <Outlet />
      </FluentProvider>
    </IdPrefixProvider>
  );
}
