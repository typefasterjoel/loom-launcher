import {
  createMemoryHistory,
  createRouter,
  RouterProvider,
} from "@tanstack/react-router";
import "./App.css";
import TitleBar from "./components/features/titlebar";

import { routeTree } from "./routeTree.gen";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 1000 * 60 * 5,
      refetchOnWindowFocus: false,
    },
  },
});

const memoryHistory = createMemoryHistory({
  initialEntries: ["/"],
});

const router = createRouter({
  routeTree,
  history: memoryHistory,
  context: {
    queryClient,
  },
  defaultPreload: "intent",
});

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <main
        id="app-shell"
        className="bg-card text-foreground flex h-screen w-screen flex-col"
      >
        <TitleBar />
        <RouterProvider router={router} />
      </main>
    </QueryClientProvider>
  );
}

export default App;
