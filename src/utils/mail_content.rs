pub fn subject() -> String {
    "Sending email with Rust".to_string()
}

pub fn mail_content() -> String {
    r#"<body>
    <div style="font-family: Arial, Helvetica, sans-serif; padding: 10px; background-color: #002147; display: flex; align-items: center; justify-content: center; min-height: 100vh;">
        <div style="max-width: 600px; margin: 0 auto; padding: 20px; background-color: #ffffff; border-radius: 10px; line-height: 1.6; border: 1px solid #c7c3c3;">
            <div style="text-align: center;">
                <img src="https://github.com/FossMec/mailer/blob/master/fossmec%20round%20logo.png?raw=true" style="width: 80px; height: 80px; margin-right: 10px;">
                <h1 style="color: #F5A212; margin: 0;">FOSSMEC</h1>
            </div>
            <div style="text-align: center; margin-top: 20px;">
                <h3>You've been shortlisted for the next round</h3>
            </div>
            <p>Dear _,</p>

            <p><strong>Congratulations!</strong> We are pleased to inform you that you have been shortlisted for the next round of selection for the <strong>[] team</strong> of FOSS MEC 2024. Your passion and skills have impressed us, and we are excited to see how you can contribute further.</p>

            <p>To ensure you stay connected with the team and up-to-date with all our plans, we've created a WhatsApp group where we'll be coordinating our efforts and sharing important information. This will be our primary platform for communication, so we encourage you to join the group as soon as possible.</p>

            <p>Please join the WhatsApp group to stay informed about our upcoming meetings and discussions. We'll be sharing important updates and details there!</p>
            <div style="text-align: left; margin: 20px 0;">
                <a href="https://www.example.com/join-group" style="text-decoration: none;">
                    <button style="background-color: #F5A212; border: none; color: white; padding: 13px 15px; text-align: center; font-size: 0.8em; cursor: pointer; border-radius: 6px;">
                        <strong>Join the group</strong>
                    </button>
                </a>
            </div>
            <p>Once again, congratulations on being shortlisted, and we look forward to seeing you in the next round!</p>
            <p>May the foss be with you</p>
            <p>Best regards,</p>
            <p style="color: #888888;">
                FOSSMEC<br>
                Open Source Cell of Model Engineering College<br>
                foss@mec.ac.in
            </p>
        </div>
    </div>
</body>

 "#
    .to_string()
}
