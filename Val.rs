export async function postApiResponse(apiUrl: string, action: string, object: string): Promise<string | null> {
    try {
        const response = await fetch(apiUrl, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ action, object }),
        });

        const jsonResponse = await response.json();

        // Ensure the response is of type RestTestDataResponse
        if (jsonResponse.success && jsonResponse.data && jsonResponse.data.otp) {
            return jsonResponse.data.otp; // Extract OTP from data
        }

        return null; // Return null if not successful or OTP is missing
    } catch (error) {
        console.error('Error in postApiResponse:', error);
        return null;
    }
}
