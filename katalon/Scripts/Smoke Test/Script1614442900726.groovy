import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://blog.stefanusfebri.com/')

WebUI.click(findTestObject('Object Repository/blog.stefanusfebri.com/Page_Hi, Stranger  My name is Stefanus Febri/a_About'))

WebUI.click(findTestObject('Object Repository/blog.stefanusfebri.com/Page_About  Hi, Stranger  My name is Stefan_96308a/span_ cd home'))

WebUI.click(findTestObject('Object Repository/blog.stefanusfebri.com/Page_Hi, Stranger  My name is Stefanus Febri/a_Posts'))

WebUI.waitForPageLoad(0)

WebUI.click(findTestObject('Object Repository/blog.stefanusfebri.com/Page_Posts  Hi, Stranger  My name is Stefan_a50b88/a_Migrate to Hugo from Jekyll              _cc3f0d'))

WebUI.waitForPageLoad(0)

WebUI.scrollToElement(findTestObject('Object Repository/blog.stefanusfebri.com/Page_Migrate to Hugo from Jekyll  Hi, Stran_b5c48f/a_                Creating a New Theme'), 
    0)

WebUI.delay(3, FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/blog.stefanusfebri.com/Page_Migrate to Hugo from Jekyll  Hi, Stran_b5c48f/a_                Creating a New Theme'))

WebUI.waitForPageLoad(0)

WebUI.scrollToPosition(750, 30516)

WebUI.delay(6, FailureHandling.STOP_ON_FAILURE)

WebUI.scrollToElement(findTestObject('Object Repository/blog.stefanusfebri.com/Page_Creating a New Theme  Hi, Stranger  My_c00e20/span_ cd home'), 
    0)

WebUI.delay(3, FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/blog.stefanusfebri.com/Page_Creating a New Theme  Hi, Stranger  My_c00e20/span_ cd home'))

WebUI.delay(3, FailureHandling.STOP_ON_FAILURE)

WebUI.closeBrowser()

