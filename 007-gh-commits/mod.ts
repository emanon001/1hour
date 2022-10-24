import { Api } from "https://cdn.skypack.dev/-/@octokit/plugin-rest-endpoint-methods@v6.7.0-RRl1lE15Kk7LVJcVYsef/dist=es2019,mode=types/dist-types/types.d.ts";
import {
  Octokit as OctokitCore,
} from "https://cdn.skypack.dev/@octokit/core?dts";
import { restEndpointMethods } from "https://cdn.skypack.dev/@octokit/plugin-rest-endpoint-methods?dts";

type ApiClient = OctokitCore & Api;

const fetchTodayEvents = async (
  octokit: ApiClient,
  username: string,
): Promise<any> => {
  // TODO:
  return [];
};

const filterCommits = (events: any): any => {
  // TODO:
  return [];
};

const formatCommits = (commits: any): any => {
  // TODO:
  return [];
};

const printCommits = (commits: any): void => {
  // TODO:
};

const createOctokit = (auth: string): ApiClient => {
  const Octokit = OctokitCore.plugin(restEndpointMethods);
  return new Octokit({
    auth,
  });
};

const auth = Deno.env.get("GH_COMMITS_AUTH_TOKEN");
if (!auth) {
  throw new Error("please specify GH_COMMITS_AUTH_TOKEN");
}
const client = createOctokit(auth);
const username = "emanon001";
const events = await fetchTodayEvents(client, username);
const commits = formatCommits(filterCommits(events));
printCommits(commits);
