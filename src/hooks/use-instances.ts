import { CreateInstancePayload, Instance } from "@/types/instance";
import {
  queryOptions,
  useMutation,
  useQuery,
  useQueryClient,
} from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

export const instanceKeys = {
  all: ["instances"] as const,
};

export const getInstancesQuery = queryOptions({
  queryKey: instanceKeys.all,
  queryFn: async () => {
    return await invoke<Instance[]>("get_instances");
  },
});

export function useInstances() {
  return useQuery(getInstancesQuery);
}

export function useCreateInstance() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (payload: CreateInstancePayload) => {
      return await invoke<Instance>("create_instance", {
        name: payload.name,
        mcVersion: payload.mcVersion,
        modLoader: payload.modLoader,
        loaderVersion: payload.loaderVersion,
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: instanceKeys.all });
    },
  });
}

export function useDeleteInstance() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (instanceId: string) => {
      return await invoke("delete_instance", { instanceId });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: instanceKeys.all });
    },
  });
}
