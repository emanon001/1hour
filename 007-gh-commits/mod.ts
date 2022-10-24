const request = async <T>(token: string, query: string): Promise<T> => {
  const url = "https://api.github.com/graphql";
  const resp = await fetch(url, {
    method: "POST",
    headers: {
      "Authorization": `bearer ${token}`,
    },
    body: JSON.stringify({ query }),
  });
  return (await resp.json()).data;
};

const fetchUsername = async (
  token: string,
): Promise<string> => {
  const query = `
  query {
    viewer {
      login
    }
  }
  `;
  const res = await request<{ viewer: { login: string } }>(token, query);
  return res.viewer.login;
};

const fetchTodayCommits = async (
  token: string,
): Promise<any> => {
  // TODO
  const query = `
  `;
  const res = await request<any>(token, query);
  return res;
};

const formatCommits = (commits: any): any => {
  // TODO:
  return [];
};

const printCommits = (commits: any): void => {
  // TODO:
};

const token = Deno.env.get("GH_COMMITS_AUTH_TOKEN");
if (!token) {
  throw new Error("please specify GH_COMMITS_AUTH_TOKEN");
}
const username = await fetchUsername(token);
console.log(username);
// const events = await fetchTodayCommits(token);
// const commits = formatCommits(filterCommits(events));
// printCommits(commits);
