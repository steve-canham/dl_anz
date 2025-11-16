
EARLY STAGES OF DEVELOPMENT

<h2>Introduction</h2>
This program is designed to process <i>all</i> ANZCTR data, as supplied originally as a single XML file dowmloaded from the ANZCTR web site (at https://anzctr.org.au/TrialSearch.aspx). 
That file is fairly large, about 90MB when unzipped, and takes some time (several minutes) to load in to Libre Office Calc. Initial processing is to convert each of the sheets in the spreadsheet file to a separate CSV file (comma separated, all text fields quoted). 
These files can then be imported into corresponding tables in the sd schema of the anz database.<br/>
<br/>

***** The system is in an initial development phase only. *****

Possible future phases include <br/>
1) Processing the imported data into an MDR compatible form, in the ad schema of the anz DB. 
2) Generating corresponding JSON files in a folder, as an equivalent local data store
<br/>
There is no 'import' or 'harvest' process required here, as the source data is always an image of the entire ANZCTR dataset.

