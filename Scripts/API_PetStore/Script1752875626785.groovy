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

Response = WS.sendRequest(findTestObject('API_Mascotas/post_AddPet'))

print(Response.getResponseText())

WS.verifyElementPropertyValue(Response, 'name', 'Firulais')

WS.verifyResponseStatusCode(Response, 200)

Response1 = WS.sendRequest(findTestObject('API_Mascotas/Get_ConsultarMascota'))

print(Response1.getResponseText())

WS.verifyResponseStatusCode(Response1, 200)

Response2 = WS.sendRequest(findTestObject('API_Mascotas/Put_Actualizar'))

print(Response2.getResponseText())

WS.verifyElementPropertyValue(Response2, 'name', 'Sofka2')

WS.verifyResponseStatusCode(Response2, 200)

Response3 = WS.sendRequest(findTestObject('API_Mascotas/Get_BuscarMascota'))

print(Response3.getResponseText())

WS.verifyResponseStatusCode(Response3, 200)

