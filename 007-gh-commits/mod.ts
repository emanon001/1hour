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

const fetchTodayCommits = async (
  token: string,
  login: string,
  email: string,
): Promise<any> => {
  const query = `
  query($login: String!, $email: String!, $from: DateTime!, $to: DateTime!, $commitSince: GitTimestamp!, $commitUntil: GitTimestamp!) {
    user(login: $login) {
      contributionsCollection(from: $from, to: $to) {
        commitContributionsByRepository {
          repository {
            name,
            url,
            ref(qualifiedName: "main") {
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
  return res;
};

const formatCommits = (commits: any): any => {
  // TODO:
  return [];
};

const printCommits = (commits: any): void => {
  // TODO:
};

const env = getEnv();
const userName = await fetchLoginUserName(env.token);
// console.log(user);
const commits = await fetchTodayCommits(env.token, userName, env.email);
console.log(commits);
// const commits = formatCommits(filterCommits(events));
// printCommits(commits);
