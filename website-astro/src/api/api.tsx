export type ProblemLeetcode = {
  id: number;
  problem_name: string;
  url: string;
  has_rust: boolean;
};

export const getAllProblems = async () => {
  return (await (
    await fetch('https://areweleetcodeyet.com/api/v1/hello', {
      method: 'GET',
      headers: {
        'Content-type': 'application/json',
        Accept: 'application/json',
      },
    })
  ).json()) as ProblemLeetcode[];
};
