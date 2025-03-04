<script lang="ts">
	import BranchIndexCard from '$lib/components/branches/BranchIndexCard.svelte';
	import { BranchService } from '@gitbutler/shared/branches/branchService';
	import { getBranchReviewsForRepository } from '@gitbutler/shared/branches/branchesPreview.svelte';
	import { getContext } from '@gitbutler/shared/context';
	import Loading from '@gitbutler/shared/network/Loading.svelte';
	import { isFound } from '@gitbutler/shared/network/loadable';
	import { lookupProject } from '@gitbutler/shared/organizations/repositoryIdLookupPreview.svelte';
	import { RepositoryIdLookupService } from '@gitbutler/shared/organizations/repositoryIdLookupService';
	import { AppState } from '@gitbutler/shared/redux/store.svelte';
	import Badge from '@gitbutler/ui/Badge.svelte';
	import type { ProjectParameters } from '$lib/routing';

	interface Props {
		data: ProjectParameters;
	}

	let { data }: Props = $props();

	const repositoryIdLookupService = getContext(RepositoryIdLookupService);
	const branchService = getContext(BranchService);
	const appState = getContext(AppState);

	const repositoryId = $derived(
		lookupProject(appState, repositoryIdLookupService, data.ownerSlug, data.projectSlug)
	);
	const brancheses = $derived(
		isFound(repositoryId.current)
			? getBranchReviewsForRepository(appState, branchService, repositoryId.current.value)
			: undefined
	);
</script>

<h2>{data.ownerSlug}/{data.projectSlug}</h2>

<Loading loadable={repositoryId.current}>
	{#snippet children(repositoryId)}
		<h3>Branches shared for review <Badge>{brancheses?.current?.length || 0}</Badge></h3>

		<table class="branches-table">
			<thead>
				<tr>
					<th><div>Seq.</div></th>
					<th><div>Name</div></th>
					<th><div>UUID</div></th>
					<th><div>Branch commits</div></th>
					<th><div>Status</div></th>
					<th><div>Last update</div></th>
					<th><div>Authors</div></th>
					<th><div>Version</div></th>
				</tr>
			</thead>
			<tbody>
				{#each brancheses?.current || [] as branches, i}
					{#each branches as branch, j}
						<BranchIndexCard
							{repositoryId}
							linkParams={data}
							uuid={branch.uuid}
							roundedTop={j === 0 && i !== 0}
							roundedBottom={j === branches.length - 1}
						/>
					{/each}
				{/each}
			</tbody>
		</table>
	{/snippet}
</Loading>

<style lang="postcss">
	.branches-table {
		th {
			padding: 0;
			> div {
				text-align: left;
				padding: 16px;

				border-top: 1px solid var(--clr-border-2);
				border-bottom: 1px solid var(--clr-border-2);
				overflow: hidden;
			}

			&:first-child {
				> div {
					border-left: 1px solid var(--clr-border-2);
					border-top-left-radius: var(--radius-m);
				}
			}

			&:last-child {
				> div {
					border-right: 1px solid var(--clr-border-2);
					border-top-right-radius: var(--radius-m);
				}
			}
		}
	}
</style>
