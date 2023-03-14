export type LeetcodeQuestion = {
  id: number;
  problem_name: string;
  url: string;
  has_rust: boolean;
};

export const getAllProblems = async () => {
  return (await (
    await fetch('https://www.areweleetcodeyet.com/api/v1/all-problems', {
      method: 'GET',
      headers: {
        'Content-type': 'application/json',
        Accept: 'application/json',
      },
    })
  ).json()) as LeetcodeQuestion[];
};
