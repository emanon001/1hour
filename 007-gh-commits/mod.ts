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

type CommitsPeriod = {
  startOfDay: string;
  endOfDay: string;
};

const getCommitsPeriod = (day: string | null): CommitsPeriod => {
  const dt = day === null ? datetime() : datetime(day);
  const startOfDay = dt.startOfDay().toISO();
  const endOfDay = dt.endOfDay().toISO();
  return {
    startOfDay,
    endOfDay,
  };
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

const fetchCommits = async (
  params: {
    token: string;
    login: string;
    email: string;
    startOfDay: string;
    endOfDay: string;
  },
): Promise<CommitsByRepository[]> => {
  const {
    token,
    login,
    email,
    startOfDay,
    endOfDay,
  } = params;

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

  const variables = {
    login,
    email,
    from: startOfDay,
    to: endOfDay,
    commitSince: startOfDay,
    commitUntil: endOfDay,
  };
  // TODO: 型パラメータを指定する
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
    const commitsCount = `${outputCommits.length}commit${
      outputCommits.length > 1 ? "s" : ""
    }`;
    console.log(` [${repo.name} ${repo.url}] ${commitsCount}`);
    console.log(outputCommits.join("\n"));
  });
};

const env = getEnv();
const userName = await fetchLoginUserName(env.token);
const commitsPeriod = getCommitsPeriod(Deno.args[0] ?? null);
const commits = await fetchCommits({
  token: env.token,
  login: userName,
  email: env.email,
  ...commitsPeriod,
});
printCommits(commits);
