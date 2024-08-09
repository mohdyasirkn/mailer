pub fn subject() -> String {
    "Sending email with Rust".to_string()
}

pub fn mail_content() -> String {
    r#"<body>
    <div style="font-family: Arial, Helvetica, sans-serif; padding: 10px; background-color: #002147; display: flex; align-items: center; justify-content: center; min-height: 100vh;">
        <div style="max-width: 600px; margin: 0 auto; padding: 20px; background-color: #ffffff; border-radius: 10px; line-height: 1.6; border: 1px solid #c7c3c3;">
            <div style="text-align: center;">
                <img src="https://github.com/mohdyasirkn/mailer/blob/content/fossmec%20round%20logo.png?raw=true" style="width: 80px; height: 80px; margin-right: 10px;">
                <h1 style="color: #F5A212; margin: 0;">FOSSMEC</h1>
            </div>
            <div style="text-align: center; margin-top: 20px;">
                <h3>You've been selected to join the team</h3>
            </div>
            <p>Dear _,</p>
            <p><strong>Congratulations!</strong> We are delighted to inform you that you have been selected to join the <strong>Events team</strong> of FOSS MEC 2024! Your passion and skills have truly impressed us, and we are excited to have you on board.</p>
            <p>We look forward to working together to conduct various events and build a vibrant community of open source enthusiasts.</p>
            <p>Please join the WhatsApp group to stay informed about our upcoming meetings and discussions. We'll be sharing important updates and details there!</p>
            <div style="text-align: left; margin: 20px 0;">
                <a href="https://www.example.com/join-group" style="text-decoration: none;">
                    <button style="background-color: #F5A212; border: none; color: white; padding: 13px 15px; text-align: center; font-size: 0.8em; cursor: pointer; border-radius: 6px;">
                        <strong>Join the group</strong>
                    </button>
                </a>
            </div>
            <p>Once again, congratulations, and welcome to the team!</p>
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
