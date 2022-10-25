import { datetime } from "https://deno.land/x/ptera@v1.0.2/mod.ts";

type Env = {
  token: string;
  email: string;
};

const getEnv = (): Env => {
  const token = Deno.env.get("GH_COMMITS_AUTH_TOKEN");
  if (!token) {
    throw new Error("please specify GH_COMMITS_AUTH_TOKEN");
  }
  const email = Deno.env.get("GH_COMMITS_EMAIL");
  if (!email) {
    throw new Error("please specify GH_COMMITS_EMAIL");
  }
  return { token, email };
};

const request = async <T>(
  token: string,
  query: string,
  variables: { [key: string]: any } = {},
): Promise<T> => {
  const url = "https://api.github.com/graphql";
  const resp = await fetch(url, {
    method: "POST",
    headers: {
      "Authorization": `bearer ${token}`,
    },
    body: JSON.stringify({ query, variables }),
  });
  return (await resp.json()).data;
};

const fetchLoginUserName = async (
  token: string,
): Promise<string> => {
  const query = `
  query {
    viewer {
      login,
    }
  }
  `;
  const res = await request<{ viewer: { login: string } }>(token, query);
  return res.viewer.login;
};

type CommitsByRepository = {
  name: string;
  url: string;
  branches: {
    name: string;
    commits: {
      message: string;
      commitUrl: string;
    }[];
  }[];
};

const fetchTodayCommits = async (
  token: string,
  login: string,
  email: string,
): Promise<CommitsByRepository[]> => {
  const query = `
  query($login: String!, $email: String!, $from: DateTime!, $to: DateTime!, $commitSince: GitTimestamp!, $commitUntil: GitTimestamp!) {
    user(login: $login) {
      contributionsCollection(from: $from, to: $to) {
        commitContributionsByRepository {
          repository {
            name,
            url,
            refs(refPrefix: "refs/heads/", first: 100) {
              nodes {
                name,
                target {
                  ... on Commit {
                    history(
                      first: 100, 
                      author: {emails: [$email]},
                      since: $commitSince,
                      until: $commitUntil,
                    ) {
                      nodes {
                        message,
                        commitUrl,
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  `;

  const now = datetime();
  const startOfDay = now.startOfDay();
  const endOfDay = now.endOfDay();

  const variables = {
    login,
    email,
    from: startOfDay.toISO(),
    to: endOfDay.toISO(),
    commitSince: startOfDay.toISO(),
    commitUntil: endOfDay.toISO(),
  };
  const res = await request<any>(token, query, variables);
  return res.user.contributionsCollection.commitContributionsByRepository.map(
    ({ repository }: { repository: any }) => {
      return {
        name: repository.name,
        url: repository.url,
        branches: repository.refs.nodes.map((b: any) => {
          return {
            name: b.name,
            commits: b.target.history.nodes,
          };
        }),
      };
    },
  );
};

const printCommits = (repoCommits: CommitsByRepository[]): void => {
  // Scrapbox format
  repoCommits.forEach((repo) => {
    const outputCommits = repo.branches.flatMap((b) => {
      const branchName = b.name;
      const branchLabel = (branchName === "main" || branchName === "master")
        ? ""
        : `${branchName}:`;
      return b.commits.map((c) => {
        return `  ${branchLabel}[${c.message} ${c.commitUrl}]`;
      });
    });
    if (outputCommits.length === 0) return;
    console.log(` [${repo.name} ${repo.url}] ${outputCommits.length}commits`);
    console.log(outputCommits.join("\n"));
  });
};

const env = getEnv();
const userName = await fetchLoginUserName(env.token);
const commits = await fetchTodayCommits(env.token, userName, env.email);
printCommits(commits);
