// Utility function to add a delay
export const delay = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));


export async function postApiResponse(apiUrl: string, action: string, object: string) {
  await delay(2000)

  const response = await fetch(apiUrl, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ action, object }),
  });

  const jsonResponse = await response.json();
  return JSON.stringify(jsonResponse);
}

await new Promise(resolve => setTimeout(resolve, 2000));
