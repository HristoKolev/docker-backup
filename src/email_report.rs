use crate::app_config::AppConfig;
use crate::errors::GeneralError;
use crate::email;
use serde_json::json;
use handlebars::Handlebars;

pub fn render_report(app_config: &AppConfig, error: &GeneralError) -> Result<String, GeneralError> {

    let html_template = r##"
        <!DOCTYPE html>
        <html>
            <head>
                <meta http-equiv="Content-Type" content="text/html; charset=UTF-8" />

                <meta property="og:title" content="*|MC:SUBJECT|*" />

                <title>*|MC:SUBJECT|*</title>
                <style type="text/css">
                    /* Client-specific Styles */
                    #outlook a{padding:0;} /* Force Outlook to provide a "view in browser" button. */
                    body{width:100% !important;} .ReadMsgBody{width:100%;} .ExternalClass{width:100%;} /* Force Hotmail to display emails at full width */
                    body{-webkit-text-size-adjust:none;} /* Prevent Webkit platforms from changing default text sizes. */

                    /* Reset Styles */
                    body{margin:0; padding:0;}
                    img{border:0; height:auto; line-height:100%; outline:none; text-decoration:none;}
                    table td{border-collapse:collapse;}
                    #backgroundTable{height:100% !important; margin:0; padding:0; width:100% !important;}

                    /* Template Styles */

                    /* /\/\/\/\/\/\/\/\/\/\ STANDARD STYLING: COMMON PAGE ELEMENTS /\/\/\/\/\/\/\/\/\/\ */

                    /**
                    * @tab Page
                    * @section background color
                    * @tip Set the background color for your email. You may want to choose one that matches your company's branding.
                    * @theme page
                    */
                    body, #backgroundTable{
                        /*@editable*/ background-color:#FAFAFA;
                    }

                    /**
                    * @tab Page
                    * @section email border
                    * @tip Set the border for your email.
                    */
                    #templateContainer{
                        /*@editable*/ border: 1px solid #DDDDDD;
                    }

                    /**
                    * @tab Page
                    * @section heading 1
                    * @tip Set the styling for all first-level headings in your emails. These should be the largest of your headings.
                    * @style heading 1
                    */
                    h1, .h1{
                        /*@editable*/ color:#202020;
                        display:block;
                        /*@editable*/ font-family:Arial;
                        /*@editable*/ font-size:34px;
                        /*@editable*/ font-weight:bold;
                        /*@editable*/ line-height:100%;
                        margin-top:0;
                        margin-right:0;
                        margin-bottom:10px;
                        margin-left:0;
                        /*@editable*/ text-align:left;
                    }

                    /**
                    * @tab Page
                    * @section heading 2
                    * @tip Set the styling for all second-level headings in your emails.
                    * @style heading 2
                    */
                    h2, .h2{
                        /*@editable*/ color:#202020;
                        display:block;
                        /*@editable*/ font-family:Arial;
                        /*@editable*/ font-size:30px;
                        /*@editable*/ font-weight:bold;
                        /*@editable*/ line-height:100%;
                        margin-top:0;
                        margin-right:0;
                        margin-bottom:10px;
                        margin-left:0;
                        /*@editable*/ text-align:left;
                    }

                    /**
                    * @tab Page
                    * @section heading 3
                    * @tip Set the styling for all third-level headings in your emails.
                    * @style heading 3
                    */
                    h3, .h3{
                        /*@editable*/ color:#202020;
                        display:block;
                        /*@editable*/ font-family:Arial;
                        /*@editable*/ font-size:26px;
                        /*@editable*/ font-weight:bold;
                        /*@editable*/ line-height:100%;
                        margin-top:0;
                        margin-right:0;
                        margin-bottom:10px;
                        margin-left:0;
                        /*@editable*/ text-align:left;
                    }

                    /**
                    * @tab Page
                    * @section heading 4
                    * @tip Set the styling for all fourth-level headings in your emails. These should be the smallest of your headings.
                    * @style heading 4
                    */
                    h4, .h4{
                        /*@editable*/ color:#202020;
                        display:block;
                        /*@editable*/ font-family:Arial;
                        /*@editable*/ font-size:22px;
                        /*@editable*/ font-weight:bold;
                        /*@editable*/ line-height:100%;
                        margin-top:0;
                        margin-right:0;
                        margin-bottom:10px;
                        margin-left:0;
                        /*@editable*/ text-align:left;
                    }

                    /* /\/\/\/\/\/\/\/\/\/\ STANDARD STYLING: PREHEADER /\/\/\/\/\/\/\/\/\/\ */

                    /**
                    * @tab Header
                    * @section preheader style
                    * @tip Set the background color for your email's preheader area.
                    * @theme page
                    */
                    #templatePreheader{
                        /*@editable*/ background-color:#FAFAFA;
                    }

                    /**
                    * @tab Header
                    * @section preheader text
                    * @tip Set the styling for your email's preheader text. Choose a size and color that is easy to read.
                    */
                    .preheaderContent div{
                        /*@editable*/ color:#505050;
                        /*@editable*/ font-family:Arial;
                        /*@editable*/ font-size:10px;
                        /*@editable*/ line-height:100%;
                        /*@editable*/ text-align:left;
                    }

                    /**
                    * @tab Header
                    * @section preheader link
                    * @tip Set the styling for your email's preheader links. Choose a color that helps them stand out from your text.
                    */
                    .preheaderContent div a:link, .preheaderContent div a:visited, /* Yahoo! Mail Override */ .preheaderContent div a .yshortcuts /* Yahoo! Mail Override */{
                        /*@editable*/ color:#336699;
                        /*@editable*/ font-weight:normal;
                        /*@editable*/ text-decoration:underline;
                    }

                    /* /\/\/\/\/\/\/\/\/\/\ STANDARD STYLING: HEADER /\/\/\/\/\/\/\/\/\/\ */

                    /**
                    * @tab Header
                    * @section header style
                    * @tip Set the background color and border for your email's header area.
                    * @theme header
                    */
                    #templateHeader{
                        /*@editable*/ background-color:#FFFFFF;
                        /*@editable*/ border-bottom:0;
                    }

                    /**
                    * @tab Header
                    * @section header text
                    * @tip Set the styling for your email's header text. Choose a size and color that is easy to read.
                    */
                    .headerContent{
                        /*@editable*/ color:#202020;
                        /*@editable*/ font-family:Arial;
                        /*@editable*/ font-size:34px;
                        /*@editable*/ font-weight:bold;
                        /*@editable*/ line-height:100%;
                        /*@editable*/ padding:0;
                        /*@editable*/ text-align:center;
                        /*@editable*/ vertical-align:middle;
                    }

                    /**
                    * @tab Header
                    * @section header link
                    * @tip Set the styling for your email's header links. Choose a color that helps them stand out from your text.
                    */
                    .headerContent a:link, .headerContent a:visited, /* Yahoo! Mail Override */ .headerContent a .yshortcuts /* Yahoo! Mail Override */{
                        /*@editable*/ color:#336699;
                        /*@editable*/ font-weight:normal;
                        /*@editable*/ text-decoration:underline;
                    }

                    #headerImage{
                        height:auto;
                        max-width:600px;
                    }

                    /* /\/\/\/\/\/\/\/\/\/\ STANDARD STYLING: MAIN BODY /\/\/\/\/\/\/\/\/\/\ */

                    /**
                    * @tab Body
                    * @section body style
                    * @tip Set the background color for your email's body area.
                    */
                    #templateContainer, .bodyContent{
                        /*@editable*/ background-color:#FFFFFF;
                    }

                    /**
                    * @tab Body
                    * @section body text
                    * @tip Set the styling for your email's main content text. Choose a size and color that is easy to read.
                    * @theme main
                    */
                    .bodyContent div{
                        /*@editable*/ color:#505050;
                        /*@editable*/ font-family:Arial;
                        /*@editable*/ font-size:14px;
                        /*@editable*/ line-height:150%;
                        /*@editable*/ text-align:left;
                    }

                    /**
                    * @tab Body
                    * @section body link
                    * @tip Set the styling for your email's main content links. Choose a color that helps them stand out from your text.
                    */
                    .bodyContent div a:link, .bodyContent div a:visited, /* Yahoo! Mail Override */ .bodyContent div a .yshortcuts /* Yahoo! Mail Override */{
                        /*@editable*/ color:#336699;
                        /*@editable*/ font-weight:normal;
                        /*@editable*/ text-decoration:underline;
                    }

                    .bodyContent img{
                        display:inline;
                        height:auto;
                    }

                    /* /\/\/\/\/\/\/\/\/\/\ STANDARD STYLING: COLUMNS; LEFT, CENTER, RIGHT /\/\/\/\/\/\/\/\/\/\ */

                    /**
                    * @tab Columns
                    * @section left column text
                    * @tip Set the styling for your email's left column text. Choose a size and color that is easy to read.
                    */
                    .leftColumnContent{
                        /*@editable*/ background-color:#FFFFFF;
                    }

                    /**
                    * @tab Columns
                    * @section left column text
                    * @tip Set the styling for your email's left column text. Choose a size and color that is easy to read.
                    */
                    .leftColumnContent div{
                        /*@editable*/ color:#505050;
                        /*@editable*/ font-family:Arial;
                        /*@editable*/ font-size:12px;
                        /*@editable*/ line-height:150%;
                        /*@editable*/ text-align:left;
                    }

                    /**
                    * @tab Columns
                    * @section left column link
                    * @tip Set the styling for your email's left column links. Choose a color that helps them stand out from your text.
                    */
                    .leftColumnContent div a:link, .leftColumnContent div a:visited, /* Yahoo! Mail Override */ .leftColumnContent div a .yshortcuts /* Yahoo! Mail Override */{
                        /*@editable*/ color:#336699;
                        /*@editable*/ font-weight:normal;
                        /*@editable*/ text-decoration:underline;
                    }

                    .leftColumnContent img{
                        display:inline;
                        height:auto;
                    }

                    /**
                    * @tab Columns
                    * @section center column text
                    * @tip Set the styling for your email's center column text. Choose a size and color that is easy to read.
                    */
                    .centerColumnContent{
                        /*@editable*/ background-color:#FFFFFF;
                    }

                    /**
                    * @tab Columns
                    * @section center column text
                    * @tip Set the styling for your email's center column text. Choose a size and color that is easy to read.
                    */
                    .centerColumnContent div{
                        /*@editable*/ color:#505050;
                        /*@editable*/ font-family:Arial;
                        /*@editable*/ font-size:14px;
                        /*@editable*/ line-height:150%;
                        /*@editable*/ text-align:left;
                    }

                    /**
                    * @tab Columns
                    * @section center column link
                    * @tip Set the styling for your email's center column links. Choose a color that helps them stand out from your text.
                    */
                    .centerColumnContent div a:link, .centerColumnContent div a:visited, /* Yahoo! Mail Override */ .centerColumnContent div a .yshortcuts /* Yahoo! Mail Override */{
                        /*@editable*/ color:#336699;
                        /*@editable*/ font-weight:normal;
                        /*@editable*/ text-decoration:underline;
                    }

                    .centerColumnContent img{
                        display:inline;
                        height:auto;
                    }

                    /**
                    * @tab Columns
                    * @section right column text
                    * @tip Set the styling for your email's right column text. Choose a size and color that is easy to read.
                    */
                    .rightColumnContent{
                        /*@editable*/ background-color:#FFFFFF;
                    }

                    /**
                    * @tab Columns
                    * @section right column text
                    * @tip Set the styling for your email's right column text. Choose a size and color that is easy to read.
                    */
                    .rightColumnContent div{
                        /*@editable*/ color:#505050;
                        /*@editable*/ font-family:Arial;
                        /*@editable*/ font-size:12px;
                        /*@editable*/ line-height:150%;
                        /*@editable*/ text-align:left;
                    }

                    /**
                    * @tab Columns
                    * @section right column link
                    * @tip Set the styling for your email's right column links. Choose a color that helps them stand out from your text.
                    */
                    .rightColumnContent div a:link, .rightColumnContent div a:visited, /* Yahoo! Mail Override */ .rightColumnContent div a .yshortcuts /* Yahoo! Mail Override */{
                        /*@editable*/ color:#336699;
                        /*@editable*/ font-weight:normal;
                        /*@editable*/ text-decoration:underline;
                    }

                    .rightColumnContent img{
                        display:inline;
                        height:auto;
                    }

                    #monkeyRewards img{
                        max-width:190px;
                    }

                </style>
            </head>
            <body leftmargin="0" marginwidth="0" topmargin="0" marginheight="0" offset="0">
                <center>
                    <table border="0" cellpadding="0" cellspacing="0" height="100%" width="100%" id="backgroundTable">
                        <tr>
                            <td align="center" valign="top">
                                <table border="0" cellpadding="0" cellspacing="0" width="900" id="templateContainer" style="background: #e1f3ff; border-radius: .3rem; padding: 1rem;">
                                    <tr>
                                        <td align="center" valign="top">
                                             An error occurred while running `docker-backup` on <span style='color: red;'>{{app_config.hostname}}</span>.
                                        </td>
                                    </tr>

                                    <tr>
                                        <td align="" valign="top">
                                            <b>Error</b>:<pre style='border: 1px solid gray; padding: 5px;'>{{formatted_error}}</pre>
                                        </td>
                                    </tr>
                                </table>
                            </td>
                        </tr>
                    </table>
                </center>
            </body>
        </html>
    "##;

    let registry = Handlebars::new();

    let rendered = registry.render_template(
        html_template,
        &json!({
            "app_config": app_config,
            "formatted_error": format!("{:#?}", error)
         })
    )?;

    Ok(rendered)
}

pub fn send_report(app_config: &AppConfig, error: &GeneralError) -> Result<(), GeneralError> {

    let subject = format!(
        "[FAILURE] An error occurred while running `docker-backup` on `{}`.",
        app_config.hostname
    );

    let report_content = render_report(&app_config, &error)?;

    send_mail(
        &app_config,
        &*subject,
        &*report_content,
    )?;

    Ok(())
}

fn send_mail(app_config: &AppConfig, subject: &str, content: &str) -> Result<(), GeneralError> {

    let email_client = email::EmailClient::new(
        &*app_config.email_config.smtp_username,
        &*app_config.email_config.smtp_password,
        &*app_config.email_config.smtp_host,
        app_config.email_config.smtp_port,
    );

    let message = email::EmailMessage::new(
        app_config.email_config.notification_emails.clone(),
        subject,
        content,
    );

    email_client.send(&message)?;

    Ok(())
}
