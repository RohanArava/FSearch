Link To Demo Video: https://youtu.be/tsOl8vEKmCg  
Running the System:  
    Download and unzip the index .rar(https://drive.google.com/file/d/1KXeZpACbBqCB6kicn9_zA23czh62WDMi/view?usp=sharing) file and place the unzipped folder in the "source code" directory.  
    In the unzipped folder, find i_index.rar and unzip this also.  
  
    To Compile:  
        The Project is done using node and rust. So rustc, cargo and node have to be installed in the sysytem.  
        To start the backend, navigate to "retrieval" directory in "source code" and run:  
        cargo run --release ..\data\world_new.csv  
  
    To run the Pre-Compiled Binary:  
        Navigate to "source code/bin" and run:  
        ./retrieval.exe ..\data\world_new.csv  
  
    To run the frontend, open a new terminal, navigate to "client" directory in "source code" and run:  
        npm i  
        npm start  
  
    Now open a web browser and navigate to "http://localhost:3006"  
  
Functionalities:  
- Search:  
    Users can enter their search query in the search bar to get matched results.  
- Execution Details:  
    Users can see the details, like the time of the execution, at the top of the page.  
- Evaluation:  
    Clicking the Circled Check Icon in the header starts the evaluation mode.  
    Now there will be checkboxes next to the retrieved articles.  
    Check the relevant checkboxes.  
    Click the hamburger menu icon to see the P-R curve of retrieved articles.  
- Relevance Feedback:   
    Below the P-R curve, You can find the "Submit Relevance Feedback" button.  
    Click the Button to submit the feedback.  
    Close the sidebar by clicking on the hamburger menu icon again, and refresh page.  
    Now, previously selected documents will be higher up in the list.  