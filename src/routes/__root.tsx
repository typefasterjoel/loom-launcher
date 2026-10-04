import Sidebar from "@/components/features/sidebar";
import { createRootRouteWithContext, Outlet } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools";
import type { QueryClient } from "@tanstack/react-query";

interface ModloaderQueryContext {
  queryClient: QueryClient;
}

const RootLayout = () => (
  <>
    <div id="app-container" className="flex h-full items-stretch">
      <aside id="app-sidebar">
        <Sidebar />
      </aside>
      <section
        id="app-content"
        className="bg-accent h-full w-full rounded-tl-lg border-s border-t p-4"
      >
        <Outlet />
      </section>
    </div>
    <TanStackRouterDevtools position="bottom-right" />
  </>
);

export const Route = createRootRouteWithContext<ModloaderQueryContext>()({
  component: RootLayout,
});
