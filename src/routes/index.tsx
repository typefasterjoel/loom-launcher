import InstanceCard from "@/components/features/instances/instance-card";
import {
  Empty,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
} from "@/components/ui/empty";
import { getInstancesQuery, useInstances } from "@/hooks/use-instances";
import { createFileRoute } from "@tanstack/react-router";
import { SquareDashedTopSolidIcon } from "lucide-react";

export const Route = createFileRoute("/")({
  component: RouteComponent,
  loader: ({ context: { queryClient } }) => {
    return queryClient.query(getInstancesQuery);
  },
});

function RouteComponent() {
  const { data: instances = [], isLoading } = useInstances();
  return (
    <section>
      <div data-app-slot="instances" className="flex flex-col gap-4">
        {instances.length === 0 && !isLoading && (
          <Empty className="border-border border border-dashed">
            <EmptyHeader>
              <EmptyMedia variant="icon" className="bg-card">
                <SquareDashedTopSolidIcon />
              </EmptyMedia>
              <EmptyTitle>No Instances Found</EmptyTitle>
              <EmptyDescription>
                Add a New Instance from the sidebar
              </EmptyDescription>
            </EmptyHeader>
          </Empty>
        )}
        {instances.map((instance) => (
          <InstanceCard instance={instance} key={instance.id} />
        ))}
      </div>
    </section>
  );
}
