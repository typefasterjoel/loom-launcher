import InstanceCard from "@/components/features/instances/instance-card";
import { getInstancesQuery, useInstances } from "@/hooks/use-instances";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/")({
  component: RouteComponent,
  loader: ({ context: { queryClient } }) => {
    return queryClient.query(getInstancesQuery);
  },
});

function RouteComponent() {
  const { data: instances = [], isLoading } = useInstances();
  console.log(instances);
  return (
    <section>
      <div data-app-slot="instances" className="flex flex-col gap-4">
        {instances.map((instance) => (
          <InstanceCard instance={instance} key={instance.id} />
        ))}
      </div>
    </section>
  );
}
